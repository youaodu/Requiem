use iced::Task;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::http_client;
use crate::models::{self, CollectionItem, EnvironmentOption, Folder};

use super::message::Message;
use super::state::{DragState, Requiem};

impl Requiem {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MethodSelected(method) => {
                if let Some(request) = self.get_current_request_mut() {
                    request.method = method;
                }
                Task::none()
            }
            Message::UrlChanged(url) => {
                if let Some(request) = self.get_current_request_mut() {
                    request.url = url;
                }
                Task::none()
            }
            Message::EnvironmentOptionSelected(option) => {
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
            Message::SendRequest => {
                if let Some(request) = self.get_current_request().cloned() {
                    info!("Sending request: {} {}", request.method.as_str(), request.url);
                    self.loading = true;
                    Task::perform(
                        async move {
                            debug!("Executing HTTP request");
                            http_client::execute_request(&request)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::RequestSent,
                    )
                } else {
                    Task::none()
                }
            }
            Message::RequestSent(result) => {
                self.loading = false;
                match result {
                    Ok(ref response) => {
                        info!("Request completed: {} in {}ms", response.status, response.time_ms);
                        self.response = Some(response.clone());
                        // Update text editor content with response body
                        self.response_body_content = iced::widget::text_editor::Content::with_text(&response.body);
                    }
                    Err(ref e) => {
                        error!("Request failed: {}", e);
                    }
                }
                Task::none()
            }
            Message::HeaderKeyChanged(idx, key) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(header) = request.headers.get_mut(idx) {
                        header.key = key;
                    }
                }
                Task::none()
            }
            Message::HeaderValueChanged(idx, value) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(header) = request.headers.get_mut(idx) {
                        header.value = value;
                    }
                }
                Task::none()
            }
            Message::AddHeader => {
                if let Some(request) = self.get_current_request_mut() {
                    request.headers.push(models::KeyValue::new("", ""));
                }
                Task::none()
            }
            Message::RemoveHeader(idx) => {
                if let Some(request) = self.get_current_request_mut() {
                    if idx < request.headers.len() {
                        request.headers.remove(idx);
                    }
                }
                Task::none()
            }
            Message::ParamKeyChanged(idx, key) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(param) = request.query_params.get_mut(idx) {
                        param.key = key;
                    }
                }
                Task::none()
            }
            Message::ParamValueChanged(idx, value) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(param) = request.query_params.get_mut(idx) {
                        param.value = value;
                    }
                }
                Task::none()
            }
            Message::AddParam => {
                if let Some(request) = self.get_current_request_mut() {
                    request.query_params.push(models::KeyValue::new("", ""));
                }
                Task::none()
            }
            Message::RemoveParam(idx) => {
                if let Some(request) = self.get_current_request_mut() {
                    if idx < request.query_params.len() {
                        request.query_params.remove(idx);
                    }
                }
                Task::none()
            }
            Message::CookieKeyChanged(idx, key) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(cookie) = request.cookies.get_mut(idx) {
                        cookie.key = key;
                    }
                }
                Task::none()
            }
            Message::CookieValueChanged(idx, value) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(cookie) = request.cookies.get_mut(idx) {
                        cookie.value = value;
                    }
                }
                Task::none()
            }
            Message::AddCookie => {
                if let Some(request) = self.get_current_request_mut() {
                    request.cookies.push(models::KeyValue::new("", ""));
                }
                Task::none()
            }
            Message::RemoveCookie(idx) => {
                if let Some(request) = self.get_current_request_mut() {
                    if idx < request.cookies.len() {
                        request.cookies.remove(idx);
                    }
                }
                Task::none()
            }
            Message::AuthKeyChanged(idx, key) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(auth_field) = request.auth.get_mut(idx) {
                        auth_field.key = key;
                    }
                }
                Task::none()
            }
            Message::AuthValueChanged(idx, value) => {
                if let Some(request) = self.get_current_request_mut() {
                    if let Some(auth_field) = request.auth.get_mut(idx) {
                        auth_field.value = value;
                    }
                }
                Task::none()
            }
            Message::AddAuthField => {
                if let Some(request) = self.get_current_request_mut() {
                    request.auth.push(models::KeyValue::new("", ""));
                }
                Task::none()
            }
            Message::RemoveAuthField(idx) => {
                if let Some(request) = self.get_current_request_mut() {
                    if idx < request.auth.len() {
                        request.auth.remove(idx);
                    }
                }
                Task::none()
            }
            Message::BodyChanged(body) => {
                if let Some(request) = self.get_current_request_mut() {
                    // Preserve the current body format
                    request.body = match request.body {
                        models::BodyType::Json(_) => models::BodyType::Json(body),
                        models::BodyType::Xml(_) => models::BodyType::Xml(body),
                        models::BodyType::Text(_) => models::BodyType::Text(body),
                        _ => models::BodyType::Json(body),
                    };
                }
                Task::none()
            }
            Message::BodyFormatChanged(format) => {
                if let Some(request) = self.get_current_request_mut() {
                    let current_text = match &request.body {
                        models::BodyType::Json(s)
                        | models::BodyType::Xml(s)
                        | models::BodyType::Text(s) => s.clone(),
                        _ => String::new(),
                    };

                    request.body = match format {
                        models::BodyFormat::None => models::BodyType::None,
                        models::BodyFormat::Json => models::BodyType::Json(current_text.clone()),
                        models::BodyFormat::Xml => models::BodyType::Xml(current_text.clone()),
                        models::BodyFormat::Text => models::BodyType::Text(current_text.clone()),
                        models::BodyFormat::FormData => models::BodyType::FormData(vec![]),
                        models::BodyFormat::FormUrlEncoded => models::BodyType::FormUrlEncoded(vec![]),
                        models::BodyFormat::Binary => models::BodyType::Binary(vec![]),
                    };

                    // Update text editor content
                    self.request_body_content = iced::widget::text_editor::Content::with_text(&current_text);
                }
                Task::none()
            }
            Message::FormDataKeyChanged(idx, key) => {
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
            Message::FormDataValueChanged(idx, value) => {
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
            Message::AddFormDataField => {
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
            Message::RemoveFormDataField(idx) => {
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
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::ResponseTabSelected(tab) => {
                self.active_response_tab = tab;
                Task::none()
            }
            Message::SelectRequest(path) => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                if !path.is_empty() {
                    self.selected_collection = Some(path[0]);
                }
                self.context_menu = None;
                // Delegate to OpenTab which handles tab creation and selection
                self.update(Message::OpenTab(path))
            }
            Message::AddNewRequest(parent_path) => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                // Create a new draft request
                let new_request = models::Request::default();
                let request_id = new_request.id;
                let request_name = new_request.name.clone();

                // Create a new unsaved tab
                let new_tab = super::state::RequestTabItem {
                    id: request_id,
                    name: request_name.clone(),
                    request_path: None, // No path yet, not saved
                    is_modified: false,
                    is_new: true,
                    draft_request: Some(new_request),
                    parent_path: Some(parent_path.clone()), // Save parent path for later
                };

                // Add the tab and switch to it
                self.open_tabs.push(new_tab);
                self.active_tab_index = Some(self.open_tabs.len() - 1);
                self.selected_request = None; // Clear selection since this is unsaved
                self.selected_collection = Some(parent_path[0]);
                self.response = None;
                self.context_menu = None;

                Task::none()
            }
            Message::AddNewFolder(parent_path) => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                let new_folder = CollectionItem::Folder(Folder {
                    id: Uuid::new_v4(),
                    name: "New Folder".to_string(),
                    items: vec![],
                    expanded: true,
                });

                let collection_idx = parent_path[0];
                if parent_path.len() == 1 {
                    // Add to collection
                    if let Some(collection) = self.collections.get_mut(collection_idx) {
                        collection.items.push(new_folder);
                    }
                } else {
                    // Add to folder
                    if let Some(CollectionItem::Folder(folder)) = self.get_item_by_path_mut(&parent_path) {
                        folder.items.push(new_folder);
                    }
                }

                // Save collection to disk
                if let Err(e) = self.save_collection(collection_idx) {
                    error!("Failed to save collection after adding folder: {}", e);
                }

                self.context_menu = None;
                Task::none()
            }
            Message::DeleteItem(path) => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                if path.len() < 2 {
                    return Task::none();
                }

                let parent_path = &path[..path.len() - 1];
                let item_idx = path[path.len() - 1];
                let collection_idx = path[0];

                if parent_path.len() == 1 {
                    // Delete from collection
                    if let Some(collection) = self.collections.get_mut(parent_path[0]) {
                        if item_idx < collection.items.len() {
                            collection.items.remove(item_idx);

                            // Clear selection if deleted item was selected
                            if self.selected_request.as_ref() == Some(&path) {
                                self.selected_request = None;
                                self.response = None;
                            }
                        }
                    }
                } else {
                    // Delete from folder
                    if let Some(CollectionItem::Folder(folder)) = self.get_item_by_path_mut(parent_path) {
                        if item_idx < folder.items.len() {
                            folder.items.remove(item_idx);

                            if self.selected_request.as_ref() == Some(&path) {
                                self.selected_request = None;
                                self.response = None;
                            }
                        }
                    }
                }

                // Save collection to disk
                if let Err(e) = self.save_collection(collection_idx) {
                    error!("Failed to save collection after deleting item: {}", e);
                }

                self.context_menu = None;
                Task::none()
            }
            Message::RenameItem(path, new_name) => {
                let collection_idx = path[0];
                let renamed_id = if path.len() == 1 {
                    // Rename collection
                    if let Some(collection) = self.collections.get_mut(path[0]) {
                        collection.name = new_name.clone();
                        Some(collection.id)
                    } else {
                        None
                    }
                } else {
                    // Rename item within collection
                    if let Some(item) = self.get_item_by_path_mut(&path) {
                        match item {
                            CollectionItem::Request(req) => {
                                debug!("Renaming request: {} -> {}, ID: {}", req.name, new_name, req.id);
                                req.name = new_name.clone();
                                Some(req.id)
                            }
                            CollectionItem::Folder(folder) => {
                                folder.name = new_name.clone();
                                Some(folder.id)
                            }
                        }
                    } else {
                        None
                    }
                };

                // Update tab names if this request is open in any tabs (match by ID)
                if let Some(id) = renamed_id {
                    debug!("Looking for tabs with ID: {}", id);
                    debug!("Open tabs: {:?}", self.open_tabs.iter().map(|t| (t.id, &t.name)).collect::<Vec<_>>());

                    let mut updated = false;
                    for tab in &mut self.open_tabs {
                        if tab.id == id {
                            debug!("Updating tab name: {} -> {}", tab.name, new_name);
                            tab.name = new_name.clone();
                            updated = true;
                        }
                    }

                    if !updated {
                        debug!("No matching tab found for ID: {}", id);
                    }
                }

                // Save collection to disk
                if let Err(e) = self.save_collection(collection_idx) {
                    error!("Failed to save collection after renaming: {}", e);
                }

                self.renaming_item = None;
                self.context_menu = None;
                Task::none()
            }
            Message::ToggleExpanded(path) => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                if path.len() == 1 {
                    // Toggle collection
                    if let Some(collection) = self.collections.get_mut(path[0]) {
                        collection.expanded = !collection.expanded;
                    }
                } else {
                    // Toggle folder
                    if let Some(CollectionItem::Folder(folder)) = self.get_item_by_path_mut(&path) {
                        folder.expanded = !folder.expanded;
                    }
                }
                Task::none()
            }
            Message::AddNewCollection => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                let default_name = format!("New Collection {}", self.collections.len() + 1);
                let new_collection = models::Collection {
                    id: Uuid::new_v4(),
                    name: default_name.clone(),
                    items: vec![],
                    expanded: true,
                };
                self.collections.push(new_collection);
                let new_coll_idx = self.collections.len() - 1;
                self.selected_collection = Some(new_coll_idx);
                self.selected_request = None;

                // Save the new collection to disk
                if let Err(e) = self.save_collection(new_coll_idx) {
                    error!("Failed to save new collection: {}", e);
                }

                // Set renaming_item to make the new collection editable
                self.renaming_item = Some((vec![new_coll_idx], default_name.clone(), default_name));
                Task::none()
            }
            Message::ShowContextMenu(path, _x, _y, target) => {
                // Use the tracked mouse position instead of passed coordinates
                let (x, y) = self.mouse_position;
                self.context_menu = Some(super::state::ContextMenu { path, x, y, target });
                Task::none()
            }
            Message::HideContextMenu => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }
                self.context_menu = None;
                Task::none()
            }
            Message::StartRename(path) => {
                use iced::widget::text_input;

                // Get the current name of the item
                let name = if path.len() == 1 {
                    // Collection
                    self.collections.get(path[0]).map(|c| c.name.clone())
                } else {
                    // Item within collection
                    self.get_item_by_path(&path).map(|item| {
                        match item {
                            CollectionItem::Request(req) => req.name.clone(),
                            CollectionItem::Folder(folder) => folder.name.clone(),
                        }
                    })
                }.unwrap_or_default();

                self.renaming_item = Some((path, name.clone(), name));
                self.context_menu = None;

                // Focus and select all text in the input
                Task::batch([
                    text_input::focus(self.rename_input_id.clone()),
                    text_input::select_all(self.rename_input_id.clone()),
                ])
            }
            Message::UpdateRenamingText(new_text) => {
                if let Some((path, original_name, _)) = &self.renaming_item {
                    self.renaming_item = Some((path.clone(), original_name.clone(), new_text));
                }
                Task::none()
            }
            Message::ConfirmRename => {
                if let Some((path, original_name, current_name)) = self.renaming_item.clone() {
                    self.renaming_item = None;
                    // Only save if the name has changed
                    if current_name != original_name && !current_name.trim().is_empty() {
                        return self.update(Message::RenameItem(path, current_name));
                    }
                }
                Task::none()
            }
            Message::CancelRename => {
                self.renaming_item = None;
                Task::none()
            }
            Message::SaveRequest => {
                // Check if current tab is a new unsaved request
                let mut collection_idx_to_save = None;
                if let Some(active_idx) = self.active_tab_index {
                    if let Some(tab) = self.open_tabs.get_mut(active_idx) {
                        if tab.is_new {
                            // Save the draft request to the collection
                            if let (Some(draft_request), Some(parent_path)) =
                                (tab.draft_request.clone(), tab.parent_path.clone()) {

                                let new_item = CollectionItem::Request(draft_request.clone());

                                // Determine where to add the request based on parent_path
                                let new_path = if parent_path.len() == 1 {
                                    // Parent is a collection
                                    let collection_idx = parent_path[0];
                                    if let Some(collection) = self.collections.get_mut(collection_idx) {
                                        collection.items.push(new_item);
                                        let new_idx = collection.items.len() - 1;
                                        collection_idx_to_save = Some(collection_idx);
                                        vec![collection_idx, new_idx]
                                    } else {
                                        // Fallback if collection not found
                                        vec![]
                                    }
                                } else {
                                    // Parent is a folder - navigate to it and add the item
                                    let collection_idx = parent_path[0];
                                    if let Some(collection) = self.collections.get_mut(collection_idx) {
                                        // Helper function to navigate and add item
                                        fn add_to_folder(
                                            items: &mut Vec<CollectionItem>,
                                            path: &[usize],
                                            new_item: CollectionItem,
                                        ) -> Option<Vec<usize>> {
                                            if path.is_empty() {
                                                items.push(new_item);
                                                Some(vec![items.len() - 1])
                                            } else {
                                                let idx = path[0];
                                                if let Some(CollectionItem::Folder(folder)) = items.get_mut(idx) {
                                                    add_to_folder(&mut folder.items, &path[1..], new_item)
                                                        .map(|mut sub_path| {
                                                            let mut result = vec![idx];
                                                            result.append(&mut sub_path);
                                                            result
                                                        })
                                                } else {
                                                    None
                                                }
                                            }
                                        }

                                        if let Some(relative_path) = add_to_folder(
                                            &mut collection.items,
                                            &parent_path[1..],
                                            new_item,
                                        ) {
                                            let mut new_path = vec![collection_idx];
                                            new_path.extend(relative_path);
                                            collection_idx_to_save = Some(collection_idx);
                                            new_path
                                        } else {
                                            vec![]
                                        }
                                    } else {
                                        vec![]
                                    }
                                };

                                if !new_path.is_empty() {
                                    // Update tab to reference the saved request
                                    tab.request_path = Some(new_path.clone());
                                    tab.is_new = false;
                                    tab.draft_request = None;
                                    tab.parent_path = None;

                                    // Update selected request
                                    self.selected_request = Some(new_path);

                                    info!("New request saved to parent path: {:?}", parent_path);
                                }
                            }
                        } else if let Some(request_path) = &tab.request_path {
                            // For existing requests, just mark collection for saving
                            collection_idx_to_save = Some(request_path[0]);
                        }
                    }
                }

                // Save collection to disk if there was a change
                if let Some(idx) = collection_idx_to_save {
                    if let Err(e) = self.save_collection(idx) {
                        error!("Failed to save collection after saving request: {}", e);
                    }
                }

                let toast = crate::ui::toast::Toast::success("Request saved successfully");
                self.toast = Some(toast.clone());

                // Auto-hide toast after duration
                let duration = toast.duration;
                Task::perform(
                    async move {
                        tokio::time::sleep(duration).await;
                    },
                    |_| Message::HideToast,
                )
            }
            Message::ShowToast(toast) => {
                info!("Showing toast: {}", toast.message);
                let duration = toast.duration;
                self.toast = Some(toast);

                // Auto-hide toast after duration
                Task::perform(
                    async move {
                        tokio::time::sleep(duration).await;
                    },
                    |_| Message::HideToast,
                )
            }
            Message::HideToast => {
                self.toast = None;
                Task::none()
            }
            Message::ShowEnvironmentDialog => {
                self.show_environment_dialog = true;
                info!("Opening environment management dialog");
                Task::none()
            }
            Message::CloseEnvironmentDialog => {
                self.show_environment_dialog = false;
                info!("Closing environment management dialog");
                Task::none()
            }
            Message::ShowSettingsDialog => {
                self.show_settings_dialog = true;
                info!("Opening settings dialog");
                Task::none()
            }
            Message::CloseSettingsDialog => {
                self.show_settings_dialog = false;
                info!("Closing settings dialog");
                Task::none()
            }
            Message::SaveDirectoryChanged(path) => {
                self.save_directory = path.clone();
                info!("Save directory changed to: {}", self.save_directory);

                // Save to config file
                if let Err(e) = crate::config::Config::load().set_save_directory(path) {
                    error!("Failed to save config: {}", e);
                }

                Task::none()
            }
            Message::BrowseSaveDirectory => {
                info!("Opening directory picker");
                // Use native file dialog to select directory
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
                            Message::HideContextMenu // No-op message
                        }
                    }
                )
            }
            Message::CopyResponseBody => {
                if let Some(response) = &self.response {
                    use iced::clipboard;
                    info!("Copying response body to clipboard");
                    clipboard::write(response.body.clone())
                } else {
                    Task::none()
                }
            }
            Message::RequestBodyAction(action) => {
                // Allow all editing actions for request body
                self.request_body_content.perform(action.clone());

                // Get text before borrowing request mutably
                let body_text = self.request_body_content.text();

                // Sync text editor content to request body
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
            Message::ResponseBodyAction(action) => {
                use iced::widget::text_editor::Action;
                // Allow all navigation and selection actions, but block editing actions
                match action {
                    // Allow all movement and selection operations
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
                    // Block all editing operations
                    Action::Edit(_) => {
                        // Ignore editing actions in read-only mode
                    }
                }
                Task::none()
            }
            Message::LanguageChanged(language) => {
                info!("Language changed to: {:?}", language);
                self.language = language;
                self.translations = crate::i18n::Translations::new(language);

                // Save to config file
                if let Err(e) = crate::config::Config::load().set_language(language) {
                    error!("Failed to save config: {}", e);
                }

                Task::none()
            }
            Message::MouseMoved(x, y) => {
                self.mouse_position = (x, y);

                // If tab is pressed, check if we should start dragging
                if let Some(press_state) = self.tab_press_state.as_ref() {
                    let total_movement = x - press_state.initial_x;

                    // If not dragging yet and movement exceeds threshold, start dragging
                    if self.drag_state.is_none() && total_movement.abs() > 20.0 {
                        info!("=== STARTING DRAG: tab_index={}, movement={:.1}px ===", press_state.tab_index, total_movement);
                        self.drag_state = Some(DragState {
                            dragging_tab_index: press_state.tab_index,
                            hover_index: Some(press_state.tab_index), // Start with current position
                            initial_mouse_x: press_state.initial_x,
                            current_mouse_x: x,
                        });
                        // Clear tab press state since we're now dragging
                        self.tab_press_state = None;
                    }
                }

                // If dragging, update mouse position and calculate hover target
                if let Some(drag_state) = self.drag_state.as_mut() {
                    drag_state.current_mouse_x = x;

                    // Calculate movement from initial tab position
                    let movement = x - drag_state.initial_mouse_x;
                    const TAB_WIDTH: f32 = 150.0; // Approximate tab width

                    // Increased hysteresis threshold to 70% of tab width to prevent flickering
                    // Must move past 70% of next tab before switching positions
                    const HYSTERESIS: f32 = TAB_WIDTH * 0.7;

                    // Calculate how many positions to move with strong hysteresis
                    // This prevents rapid switching when hovering near boundaries
                    let positions_moved = if movement > 0.0 {
                        ((movement - HYSTERESIS) / TAB_WIDTH).floor().max(0.0) as i32
                    } else {
                        ((movement + HYSTERESIS) / TAB_WIDTH).ceil().min(0.0) as i32
                    };

                    let dragging_idx = drag_state.dragging_tab_index as i32;
                    let mut target_idx = (dragging_idx + positions_moved).max(0) as usize;
                    target_idx = target_idx.min(self.open_tabs.len().saturating_sub(1));

                    // Update hover index for visual feedback
                    if drag_state.hover_index != Some(target_idx) {
                        info!("Hover target changed: {:?} -> {} (movement={:.1}px, positions={})",
                              drag_state.hover_index, target_idx, movement, positions_moved);
                        drag_state.hover_index = Some(target_idx);
                    }
                }

                Task::none()
            }
            Message::SearchChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::OpenTab(path) => {
                // Check if tab already exists
                if let Some(existing_idx) = self.open_tabs.iter().position(|tab| tab.request_path.as_ref() == Some(&path)) {
                    // Tab exists, just switch to it
                    self.active_tab_index = Some(existing_idx);
                    self.selected_request = Some(path);
                } else {
                    // Get request name and ID
                    let (name, id) = if let Some(item) = self.get_item_by_path(&path) {
                        match item {
                            CollectionItem::Request(req) => {
                                debug!("Opening tab for request: name={}, id={}", req.name, req.id);
                                (req.name.clone(), req.id)
                            }
                            _ => ("Unknown".to_string(), Uuid::new_v4()),
                        }
                    } else {
                        ("Unknown".to_string(), Uuid::new_v4())
                    };

                    // Create new tab with the request's actual ID
                    let new_tab = super::state::RequestTabItem {
                        id,
                        name: name.clone(),
                        request_path: Some(path.clone()),
                        is_modified: false,
                        is_new: false,
                        draft_request: None,
                        parent_path: None,
                    };

                    debug!("Created tab with id={}, name={}", id, name);
                    self.open_tabs.push(new_tab);
                    self.active_tab_index = Some(self.open_tabs.len() - 1);
                    self.selected_request = Some(path);
                }
                self.response = None;
                Task::none()
            }
            Message::CloseTab(index) => {
                info!("=== CLOSE TAB CALLED: index={} ===", index);
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                if index < self.open_tabs.len() {
                    self.open_tabs.remove(index);

                    // Update active tab index
                    if self.open_tabs.is_empty() {
                        self.active_tab_index = None;
                        self.selected_request = None;
                        self.response = None;
                    } else if let Some(active_idx) = self.active_tab_index {
                        if active_idx >= index {
                            // If we closed the active tab or one before it, adjust the active index
                            self.active_tab_index = Some(active_idx.saturating_sub(1).min(self.open_tabs.len() - 1));
                        }
                        // Update selected request to match the new active tab
                        if let Some(new_active_idx) = self.active_tab_index {
                            if let Some(tab) = self.open_tabs.get(new_active_idx) {
                                self.selected_request = tab.request_path.clone();
                            }
                        }
                    }
                }
                Task::none()
            }
            Message::SwitchTab(index) => {
                // Auto-save rename if in progress
                if self.renaming_item.is_some() {
                    let _ = self.update(Message::ConfirmRename);
                }

                if index < self.open_tabs.len() {
                    self.active_tab_index = Some(index);
                    if let Some(tab) = self.open_tabs.get(index) {
                        self.selected_request = tab.request_path.clone();
                        self.response = None;

                        // Sync request body to text editor content
                        if let Some(request) = self.get_current_request() {
                            let body_text = match &request.body {
                                models::BodyType::Json(s)
                                | models::BodyType::Xml(s)
                                | models::BodyType::Text(s) => s.clone(),
                                _ => String::new(),
                            };
                            self.request_body_content = iced::widget::text_editor::Content::with_text(&body_text);
                        }
                    }
                }
                Task::none()
            }
            Message::TabDragStart(index, _offset_x) => {
                info!("=== TAB DRAG START CALLED: index={} ===", index);
                if index < self.open_tabs.len() {
                    // Record the initial mouse X position when starting drag
                    let initial_x = self.mouse_position.0;
                    info!("Tab drag started: index={}, initial_x={}", index, initial_x);
                    self.drag_state = Some(super::state::DragState {
                        dragging_tab_index: index,
                        hover_index: Some(index),
                        initial_mouse_x: initial_x,
                        current_mouse_x: initial_x,
                    });
                }
                Task::none()
            }
            Message::TabDragMove(x) => {
                if let Some(drag_state) = &mut self.drag_state {
                    // Calculate which tab index the mouse is hovering over based on x position
                    // Simple calculation: assume each tab is ~120px wide
                    let tab_width = 120.0;
                    let hover_idx = (x / tab_width).floor() as usize;
                    drag_state.hover_index = if hover_idx < self.open_tabs.len() {
                        Some(hover_idx)
                    } else {
                        None
                    };
                }
                Task::none()
            }
            Message::TabDragEnd => {
                info!("=== TAB DRAG END CALLED (mouse_pos={}, {}) ===", self.mouse_position.0, self.mouse_position.1);

                // If we have a tab press state (not dragged, just clicked)
                if let Some(press_state) = self.tab_press_state.take() {
                    let duration = press_state.press_time.elapsed();
                    let current_x = self.mouse_position.0;
                    let x_movement = current_x - press_state.initial_x;
                    info!(
                        "Tab {} press released: duration={:.3}s, initial_x={}, current_x={}, X movement={:.1}px",
                        press_state.tab_index,
                        duration.as_secs_f64(),
                        press_state.initial_x,
                        current_x,
                        x_movement
                    );

                    // If it's a quick click (not a drag), switch to that tab
                    if x_movement.abs() < 20.0 && duration.as_millis() < 300 {
                        info!("Activating tab {} (click detected)", press_state.tab_index);
                        self.active_tab_index = Some(press_state.tab_index);
                        if let Some(tab) = self.open_tabs.get(press_state.tab_index) {
                            self.selected_request = tab.request_path.clone();
                        }
                    }
                }

                // If we were dragging, perform the actual reorder now
                if let Some(drag_state) = self.drag_state.take() {
                    if let Some(target_idx) = drag_state.hover_index {
                        let from_idx = drag_state.dragging_tab_index;
                        if from_idx != target_idx {
                            info!("Drag ended: reordering tab from {} to {}", from_idx, target_idx);
                            return self.update(Message::ReorderTabs(from_idx, target_idx));
                        }
                    }
                    info!("Drag ended without reorder");
                }

                Task::none()
            }
            Message::ReorderTabs(from_index, to_index) => {
                if from_index < self.open_tabs.len() && to_index < self.open_tabs.len() {
                    let tab = self.open_tabs.remove(from_index);
                    self.open_tabs.insert(to_index, tab);

                    // Update active tab index if needed
                    if let Some(active_idx) = self.active_tab_index {
                        if active_idx == from_index {
                            self.active_tab_index = Some(to_index);
                        } else if from_index < active_idx && to_index >= active_idx {
                            self.active_tab_index = Some(active_idx - 1);
                        } else if from_index > active_idx && to_index <= active_idx {
                            self.active_tab_index = Some(active_idx + 1);
                        }
                    }
                }
                Task::none()
            }
            Message::MoveActiveTabLeft => {
                // Arrow key left: Move tab position left (reorder)
                if let Some(active_idx) = self.active_tab_index {
                    if active_idx > 0 {
                        return self.update(Message::ReorderTabs(active_idx, active_idx - 1));
                    }
                }
                Task::none()
            }
            Message::MoveActiveTabRight => {
                // Arrow key right: Move tab position right (reorder)
                if let Some(active_idx) = self.active_tab_index {
                    if active_idx < self.open_tabs.len() - 1 {
                        return self.update(Message::ReorderTabs(active_idx, active_idx + 1));
                    }
                }
                Task::none()
            }
            Message::TabPressStart(index, _x) => {
                use crate::app::state::TabPressState;
                let initial_x = self.mouse_position.0;
                let press_time = std::time::Instant::now();
                self.tab_press_state = Some(TabPressState {
                    tab_index: index,
                    press_time,
                    initial_x,
                    last_x: initial_x,  // 初始化时,上一次位置就是初始位置
                    delta_x: 0.0,       // 初始差值为 0
                });
                info!("=== TAB PRESS START: index={}, initial_x={}, time={:?} ===", index, initial_x, press_time);
                Task::none()
            }
        }
    }
}
