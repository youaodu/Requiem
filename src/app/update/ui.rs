use iced::Task;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::models::EnvironmentOption;

use super::super::message::Message;
use super::super::state::{ContextMenu, ContextMenuTarget, DragState, Requiem};

/// Structure for parsing AI-generated request configuration
#[derive(Debug, Deserialize, Serialize)]
struct AiGeneratedRequest {
    method: String,
    url: String,
    headers: Vec<AiKeyValue>,
    params: Vec<AiKeyValue>,
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AiKeyValue {
    key: String,
    value: String,
}

impl Requiem {
    /// Handle environment option selection
    pub fn handle_environment_option_selected(
        &mut self,
        option: EnvironmentOption,
    ) -> Task<Message> {
        match option {
            EnvironmentOption::Environment(env) => {
                self.current_environment = env;
                info!("Environment changed to: {:?}", env);
                Task::none()
            }
            EnvironmentOption::ManageEnvironments => {
                info!("Opening environment management dialog");
                self.show_environment_dialog = true;
                Task::none()
            }
        }
    }

    /// Handle body view mode selection (Raw, JSON, XML, HTML)
    pub fn handle_body_view_mode_selected(
        &mut self,
        mode: crate::models::BodyViewMode,
    ) -> Task<Message> {
        self.active_body_view_mode = mode;

        let formatted_body = match mode {
            crate::models::BodyViewMode::Raw => self.raw_response_body.clone(),
            crate::models::BodyViewMode::Json => {
                match crate::utils::formatter::format_json(&self.raw_response_body) {
                    Ok(formatted) => formatted,
                    Err(_) => self.raw_response_body.clone(),
                }
            }
            crate::models::BodyViewMode::Xml => {
                match crate::utils::formatter::format_xml(&self.raw_response_body) {
                    Ok(formatted) => formatted,
                    Err(_) => self.raw_response_body.clone(),
                }
            }
            crate::models::BodyViewMode::Html => {
                match crate::utils::formatter::format_html(&self.raw_response_body) {
                    Ok(formatted) => formatted,
                    Err(_) => self.raw_response_body.clone(),
                }
            }
        };

        self.response_body_content = iced::widget::text_editor::Content::with_text(&formatted_body);
        Task::none()
    }

    /// Show context menu
    pub fn handle_show_context_menu(
        &mut self,
        path: Vec<usize>,
        _x: f32,
        _y: f32,
        target: ContextMenuTarget,
    ) -> Task<Message> {
        let (x, y) = self.mouse_position;
        self.context_menu = Some(ContextMenu { path, x, y, target });
        Task::none()
    }

    /// Hide context menu
    pub fn handle_hide_context_menu(&mut self) -> Task<Message> {
        // Auto-save rename if in progress
        if self.renaming_item.is_some() {
            let _ = self.update(Message::ConfirmRename);
        }
        self.context_menu = None;
        Task::none()
    }

    /// Show toast notification
    pub fn handle_show_toast(&mut self, toast: crate::ui::toast::Toast) -> Task<Message> {
        info!("Showing toast: {}", toast.message);
        let duration = toast.duration;
        self.toast = Some(toast);

        Task::perform(
            async move {
                tokio::time::sleep(duration).await;
            },
            |_| Message::HideToast,
        )
    }

    /// Hide toast notification
    pub fn handle_hide_toast(&mut self) -> Task<Message> {
        self.toast = None;
        Task::none()
    }

    /// Show environment dialog
    pub fn handle_show_environment_dialog(&mut self) -> Task<Message> {
        self.show_environment_dialog = true;
        info!("Opening environment management dialog");
        Task::none()
    }

    /// Close environment dialog
    pub fn handle_close_environment_dialog(&mut self) -> Task<Message> {
        self.show_environment_dialog = false;
        info!("Closing environment management dialog");
        Task::none()
    }

    /// Show settings dialog
    pub fn handle_show_settings_dialog(&mut self) -> Task<Message> {
        self.show_settings_dialog = true;
        info!("Opening settings dialog");
        Task::none()
    }

    /// Close settings dialog
    pub fn handle_close_settings_dialog(&mut self) -> Task<Message> {
        self.show_settings_dialog = false;
        info!("Closing settings dialog");
        Task::none()
    }

    /// Handle save directory change
    pub fn handle_save_directory_changed(&mut self, path: String) -> Task<Message> {
        self.save_directory = path.clone();
        info!("Save directory changed to: {}", self.save_directory);

        if let Err(e) = crate::config::Config::load().set_save_directory(path) {
            error!("Failed to save config: {}", e);
        }

        Task::none()
    }

    /// Browse for save directory
    pub fn handle_browse_save_directory(&mut self) -> Task<Message> {
        info!("Opening directory picker");
        Task::perform(
            async {
                use rfd::AsyncFileDialog;
                AsyncFileDialog::new()
                    .set_title("Select Save Directory")
                    .pick_folder()
                    .await
            },
            |result| {
                if let Some(folder) = result {
                    Message::SaveDirectoryChanged(folder.path().to_string_lossy().to_string())
                } else {
                    Message::HideContextMenu
                }
            },
        )
    }

    /// Handle language change
    pub fn handle_language_changed(&mut self, language: crate::i18n::Language) -> Task<Message> {
        info!("Language changed to: {:?}", language);
        self.language = language;
        self.translations = crate::i18n::Translations::new(language);

        if let Err(e) = crate::config::Config::load().set_language(language) {
            error!("Failed to save config: {}", e);
        }

        Task::none()
    }

    /// Handle AI engine change
    pub fn handle_ai_engine_changed(&mut self, engine: crate::models::AiEngine) -> Task<Message> {
        info!("AI engine changed to: {:?}", engine);
        self.ai_config.engine = engine;

        if let Err(e) = crate::config::Config::load().set_ai_config(self.ai_config.clone()) {
            error!("Failed to save config: {}", e);
        }

        Task::none()
    }

    /// Handle AI API URL change
    pub fn handle_ai_api_url_changed(&mut self, url: String) -> Task<Message> {
        info!("AI API URL changed to: {}", url);
        self.ai_config.openai_config.api_url = url;

        if let Err(e) = crate::config::Config::load().set_ai_config(self.ai_config.clone()) {
            error!("Failed to save config: {}", e);
        }

        Task::none()
    }

    /// Handle AI API key change
    pub fn handle_ai_api_key_changed(&mut self, key: String) -> Task<Message> {
        info!("AI API key changed");
        self.ai_config.openai_config.api_key = key;

        if let Err(e) = crate::config::Config::load().set_ai_config(self.ai_config.clone()) {
            error!("Failed to save config: {}", e);
        }

        Task::none()
    }

    /// Handle AI model change
    pub fn handle_ai_model_changed(&mut self, model: String) -> Task<Message> {
        info!("AI model changed to: {}", model);
        self.ai_config.openai_config.model = model;

        if let Err(e) = crate::config::Config::load().set_ai_config(self.ai_config.clone()) {
            error!("Failed to save config: {}", e);
        }

        Task::none()
    }

    /// Handle mouse movement
    pub fn handle_mouse_moved(&mut self, x: f32, y: f32) -> Task<Message> {
        self.mouse_position = (x, y);

        // Check if we should start dragging from tab press
        if let Some(press_state) = self.tab_press_state.as_ref() {
            let total_movement = x - press_state.initial_x;

            if self.drag_state.is_none() && total_movement.abs() > 20.0 {
                info!(
                    "=== STARTING DRAG: tab_index={}, movement={:.1}px ===",
                    press_state.tab_index, total_movement
                );
                self.drag_state = Some(DragState {
                    dragging_tab_index: press_state.tab_index,
                    hover_index: Some(press_state.tab_index),
                    initial_mouse_x: press_state.initial_x,
                    current_mouse_x: x,
                });
                self.tab_press_state = None;
            }
        }

        // Update drag state if dragging
        if let Some(drag_state) = self.drag_state.as_mut() {
            drag_state.current_mouse_x = x;

            let movement = x - drag_state.initial_mouse_x;
            const TAB_WIDTH: f32 = 150.0;
            const HYSTERESIS: f32 = TAB_WIDTH * 0.7;

            let positions_moved = if movement > 0.0 {
                ((movement - HYSTERESIS) / TAB_WIDTH).floor().max(0.0) as i32
            } else {
                ((movement + HYSTERESIS) / TAB_WIDTH).ceil().min(0.0) as i32
            };

            let dragging_idx = drag_state.dragging_tab_index as i32;
            let mut target_idx = (dragging_idx + positions_moved).max(0) as usize;
            target_idx = target_idx.min(self.open_tabs.len().saturating_sub(1));

            if drag_state.hover_index != Some(target_idx) {
                info!(
                    "Hover target changed: {:?} -> {} (movement={:.1}px, positions={})",
                    drag_state.hover_index, target_idx, movement, positions_moved
                );
                drag_state.hover_index = Some(target_idx);
            }
        }

        Task::none()
    }

    /// Handle search query change
    pub fn handle_search_changed(&mut self, query: String) -> Task<Message> {
        self.search_query = query;
        Task::none()
    }

    /// Extract JSON from AI response, removing markdown code blocks if present
    fn extract_json_from_response(response: &str) -> String {
        let trimmed = response.trim();

        // Check if wrapped in markdown code block
        if trimmed.starts_with("```") {
            // Find the first newline after opening ```
            if let Some(start_pos) = trimmed.find('\n') {
                let content = &trimmed[start_pos + 1..];

                // Find closing ```
                if let Some(end_pos) = content.rfind("```") {
                    return content[..end_pos].trim().to_string();
                }
            }
        }

        // If no code block, try to find JSON object bounds
        if let Some(start) = trimmed.find('{') {
            if let Some(end) = trimmed.rfind('}') {
                if end >= start {
                    return trimmed[start..=end].to_string();
                }
            }
        }

        // Return as-is if no patterns matched
        trimmed.to_string()
    }

    /// Show AI Fill dialog
    pub fn handle_show_ai_fill_dialog(&mut self) -> Task<Message> {
        self.show_ai_fill_dialog = true;
        self.ai_fill_input_content = iced::widget::text_editor::Content::new();
        info!("Opening AI Fill dialog");
        Task::none()
    }

    /// Close AI Fill dialog
    pub fn handle_close_ai_fill_dialog(&mut self) -> Task<Message> {
        self.show_ai_fill_dialog = false;
        self.ai_fill_input_content = iced::widget::text_editor::Content::new();
        info!("Closing AI Fill dialog");
        Task::none()
    }

    /// Handle AI Fill input text editor action
    pub fn handle_ai_fill_input_action(
        &mut self,
        action: iced::widget::text_editor::Action,
    ) -> Task<Message> {
        self.ai_fill_input_content.perform(action);
        Task::none()
    }

    /// Confirm AI Fill (process the input)
    pub fn handle_confirm_ai_fill(&mut self) -> Task<Message> {
        let input_text = self.ai_fill_input_content.text();

        if input_text.trim().is_empty() {
            return Task::none();
        }

        info!("AI Fill confirmed with input: {} chars", input_text.len());

        // Set loading state (keep dialog open)
        self.ai_fill_loading = true;
        let user_prompt = input_text.clone();

        // Check AI engine and call appropriate API
        use crate::models::AiEngine;
        match self.ai_config.engine {
            AiEngine::OpenAI => {
                let api_url = self.ai_config.openai_config.api_url.clone();
                let api_key = self.ai_config.openai_config.api_key.clone();
                let model = self.ai_config.openai_config.model.clone();

                // Validate configuration
                if api_key.trim().is_empty() {
                    self.ai_fill_loading = false;
                    return Task::done(Message::ShowToast(crate::ui::toast::Toast::error(
                        "OpenAI API Key is not configured".to_string(),
                    )));
                }

                // Call OpenAI API asynchronously
                Task::perform(
                    async move {
                        crate::http_client::call_openai_api(
                            &api_url,
                            &api_key,
                            &model,
                            &user_prompt,
                        )
                        .await
                    },
                    |result| Message::AiFillCompleted(result.map_err(|e| e.to_string())),
                )
            }
            AiEngine::ClaudeCode | AiEngine::Codex => {
                // Call AI client in a separate task
                // Note: We can't easily pass AiClient through Message because it's not Clone
                // So we'll create a temporary client for this request
                let engine = self.ai_config.engine.clone();

                // Build full prompt with system instructions
                let full_prompt = format!(
                    "You are an API request generator assistant. Your task is to generate complete HTTP API request configurations based on user descriptions.\n\
                    \n\
                    Rules:\n\
                    - Return ONLY a valid JSON object, no explanations or additional text\n\
                    - The JSON must contain these fields: method, url, headers, params, body\n\
                    - Use common API conventions (RESTful style)\n\
                    - Headers and params should be arrays of {{\"key\": \"...\", \"value\": \"...\"}} objects\n\
                    - Body should be a JSON string (empty string if no body needed)\n\
                    \n\
                    JSON Format:\n\
                    {{\n\
                      \"method\": \"GET|POST|PUT|PATCH|DELETE\",\n\
                      \"url\": \"https://api.example.com/path\",\n\
                      \"headers\": [{{\"key\": \"Content-Type\", \"value\": \"application/json\"}}],\n\
                      \"params\": [{{\"key\": \"page\", \"value\": \"1\"}}],\n\
                      \"body\": \"{{\\\"name\\\": \\\"value\\\"}}\"\n\
                    }}\n\
                    \n\
                    User request: {}\n\
                    \n\
                    Generate the JSON:",
                    user_prompt
                );

                Task::perform(
                    async move {
                        let mut ai_client = crate::ai_client::AiClient::new(engine);
                        ai_client.send_prompt(&full_prompt).await
                    },
                    |result| Message::AiFillCompleted(result.map_err(|e| e.to_string())),
                )
            }
        }
    }

    /// Handle AI Fill completion
    pub fn handle_ai_fill_completed(&mut self, result: Result<String, String>) -> Task<Message> {
        // Close dialog and clear loading state
        self.show_ai_fill_dialog = false;
        self.ai_fill_loading = false;
        self.ai_fill_input_content = iced::widget::text_editor::Content::new();

        match result {
            Ok(generated_response) => {
                info!("AI Fill completed successfully: {}", generated_response);

                // Extract JSON from response (may be wrapped in markdown code block)
                let json_str = Self::extract_json_from_response(&generated_response);

                // Parse JSON response
                let ai_request: AiGeneratedRequest = match serde_json::from_str(&json_str) {
                    Ok(req) => req,
                    Err(e) => {
                        error!("Failed to parse AI response as JSON: {}", e);
                        error!("Extracted JSON: {}", json_str);
                        return Task::done(Message::ShowToast(crate::ui::toast::Toast::error(
                            format!("Failed to parse AI response: {}", e),
                        )));
                    }
                };

                // Get current request based on active tab
                if let Some(current_request) = self.get_current_request_mut() {
                    // Parse and set HTTP method
                    if let Ok(method) = ai_request
                        .method
                        .to_uppercase()
                        .parse::<crate::models::HttpMethod>()
                    {
                        current_request.method = method;
                    }

                    // Set URL
                    current_request.url = ai_request.url.trim().to_string();

                    // Set headers - convert AiKeyValue to KeyValue
                    current_request.headers = ai_request
                        .headers
                        .into_iter()
                        .map(|h| crate::models::KeyValue::new(h.key, h.value))
                        .collect();

                    // Set query params
                    current_request.query_params = ai_request
                        .params
                        .into_iter()
                        .map(|p| crate::models::KeyValue::new(p.key, p.value))
                        .collect();

                    // Set body (as JSON)
                    if !ai_request.body.is_empty() {
                        current_request.body =
                            crate::models::BodyType::Json(ai_request.body.clone());

                        // Update the request body content editor
                        self.request_body_content =
                            iced::widget::text_editor::Content::with_text(&ai_request.body);
                    }

                    Task::done(Message::ShowToast(crate::ui::toast::Toast::success(
                        "AI Fill completed successfully".to_string(),
                    )))
                } else {
                    Task::done(Message::ShowToast(crate::ui::toast::Toast::warning(
                        "No active request to fill".to_string(),
                    )))
                }
            }
            Err(error) => {
                error!("AI Fill failed: {}", error);
                Task::done(Message::ShowToast(crate::ui::toast::Toast::error(format!(
                    "AI Fill failed: {}",
                    error
                ))))
            }
        }
    }
}
