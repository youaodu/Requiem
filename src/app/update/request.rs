use iced::Task;
use tracing::{debug, error, info};

use crate::models;

use super::super::message::Message;
use super::super::state::Requiem;

impl Requiem {
    /// Handle request method selection
    pub fn handle_method_selected(&mut self, method: models::HttpMethod) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            request.method = method;
        }
        Task::none()
    }

    /// Handle URL changes
    pub fn handle_url_changed(&mut self, url: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            request.url = url;
        }
        Task::none()
    }

    /// Handle AI Fill action
    pub fn handle_ai_fill(&mut self) -> Task<Message> {
        info!("AI Fill triggered - opening dialog");
        Task::done(Message::ShowAiFillDialog)
    }

    /// Handle send request action
    pub fn handle_send_request(&mut self) -> Task<Message> {
        if let Some(request) = self.get_current_request().cloned() {
            info!(
                "Sending request: {} {}",
                request.method.as_str(),
                request.url
            );
            self.loading = true;
            Task::perform(
                async move {
                    debug!("Executing HTTP request");
                    crate::http_client::execute_request(&request)
                        .await
                        .map_err(|e| e.to_string())
                },
                Message::RequestSent,
            )
        } else {
            Task::none()
        }
    }

    /// Handle request completion
    pub fn handle_request_sent(
        &mut self,
        result: Result<models::Response, String>,
    ) -> Task<Message> {
        self.loading = false;
        match result {
            Ok(ref response) => {
                info!(
                    "Request completed: {} in {}ms",
                    response.status, response.time_ms
                );
                self.response = Some(response.clone());
                self.raw_response_body = response.body.clone();
                self.response_body_content =
                    iced::widget::text_editor::Content::with_text(&response.body);
                self.active_body_view_mode = crate::models::BodyViewMode::Raw;
                self.error_message = None; // Clear any previous error
            }
            Err(ref e) => {
                error!("Request failed: {}", e);
                self.error_message = Some(e.clone()); // Store error message
                self.response = None; // Clear response on error
            }
        }
        Task::none()
    }

    /// Handle body content changes
    pub fn handle_body_changed(&mut self, body: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            request.body = match request.body {
                models::BodyType::Json(_) => models::BodyType::Json(body),
                models::BodyType::Xml(_) => models::BodyType::Xml(body),
                models::BodyType::Text(_) => models::BodyType::Text(body),
                _ => models::BodyType::Json(body),
            };
        }
        Task::none()
    }

    /// Handle body format changes
    pub fn handle_body_format_changed(&mut self, format: models::BodyFormat) -> Task<Message> {
        // Step 1: Get request info and current text (release borrow after this block)
        let (request_id, current_format, current_text) = {
            if let Some(request) = self.get_current_request() {
                let request_id = request.id;
                let current_format = request.body.format();
                let current_text = match &request.body {
                    models::BodyType::Json(s)
                    | models::BodyType::Xml(s)
                    | models::BodyType::Text(s) => s.clone(),
                    _ => String::new(),
                };
                (request_id, current_format, current_text)
            } else {
                return Task::none();
            }
        };

        // Step 2: Save current text to cache if not empty
        if !current_text.is_empty() {
            self.body_format_cache
                .insert((request_id, current_format), current_text);
        }

        // Step 3: Restore cached content for the new format
        let restored_text = self
            .body_format_cache
            .get(&(request_id, format))
            .cloned()
            .unwrap_or_default();

        // Step 4: Update request body with new format
        if let Some(request) = self.get_current_request_mut() {
            request.body = match format {
                models::BodyFormat::None => models::BodyType::None,
                models::BodyFormat::Json => models::BodyType::Json(restored_text.clone()),
                models::BodyFormat::Xml => models::BodyType::Xml(restored_text.clone()),
                models::BodyFormat::Text => models::BodyType::Text(restored_text.clone()),
                models::BodyFormat::FormData => models::BodyType::FormData(vec![]),
                models::BodyFormat::FormUrlEncoded => models::BodyType::FormUrlEncoded(vec![]),
                models::BodyFormat::Binary => models::BodyType::Binary(vec![]),
            };
        }

        // Step 5: Update text editor content
        self.request_body_content = iced::widget::text_editor::Content::with_text(&restored_text);

        Task::none()
    }

    /// Handle form data field key changes
    pub fn handle_form_data_key_changed(&mut self, idx: usize, key: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            match &mut request.body {
                models::BodyType::FormData(fields) | models::BodyType::FormUrlEncoded(fields) => {
                    if let Some(field) = fields.get_mut(idx) {
                        field.key = key;
                    }
                }
                _ => {}
            }
        }
        Task::none()
    }

    /// Handle form data field value changes
    pub fn handle_form_data_value_changed(&mut self, idx: usize, value: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            match &mut request.body {
                models::BodyType::FormData(fields) | models::BodyType::FormUrlEncoded(fields) => {
                    if let Some(field) = fields.get_mut(idx) {
                        field.value = value;
                    }
                }
                _ => {}
            }
        }
        Task::none()
    }

    /// Add new form data field
    pub fn handle_add_form_data_field(&mut self) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            match &mut request.body {
                models::BodyType::FormData(fields) | models::BodyType::FormUrlEncoded(fields) => {
                    fields.push(models::KeyValue::new("", ""));
                }
                _ => {}
            }
        }
        Task::none()
    }

    /// Remove form data field
    pub fn handle_remove_form_data_field(&mut self, idx: usize) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            match &mut request.body {
                models::BodyType::FormData(fields) | models::BodyType::FormUrlEncoded(fields) => {
                    if idx < fields.len() {
                        fields.remove(idx);
                    }
                }
                _ => {}
            }
        }
        Task::none()
    }

    /// Handle request body editor actions
    pub fn handle_request_body_action(
        &mut self,
        action: iced::widget::text_editor::Action,
    ) -> Task<Message> {
        self.request_body_content.perform(action.clone());
        let body_text = self.request_body_content.text();

        if let Some(request) = self.get_current_request_mut() {
            request.body = match request.body {
                models::BodyType::Json(_) => models::BodyType::Json(body_text.clone()),
                models::BodyType::Xml(_) => models::BodyType::Xml(body_text.clone()),
                models::BodyType::Text(_) => models::BodyType::Text(body_text.clone()),
                _ => models::BodyType::Text(body_text),
            };
        }
        Task::none()
    }

    /// Handle response body editor actions (read-only)
    pub fn handle_response_body_action(
        &mut self,
        action: iced::widget::text_editor::Action,
    ) -> Task<Message> {
        use iced::widget::text_editor::Action;
        match action {
            Action::Move(_)
            | Action::Select(_)
            | Action::SelectWord
            | Action::SelectLine
            | Action::SelectAll
            | Action::Scroll { .. }
            | Action::Click(_)
            | Action::Drag(_) => {
                self.response_body_content.perform(action);
            }
            Action::Edit(_) => {
                // Ignore editing actions in read-only mode
            }
        }
        Task::none()
    }

    /// Copy response body to clipboard
    pub fn handle_copy_response_body(&mut self) -> Task<Message> {
        if let Some(response) = &self.response {
            use iced::clipboard;
            info!("Copying response body to clipboard");
            clipboard::write(response.body.clone())
        } else {
            Task::none()
        }
    }
}
