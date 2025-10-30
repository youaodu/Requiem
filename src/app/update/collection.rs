use iced::widget::operation::{focus, select_all};
use iced::Task;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::models::{self, CollectionItem, Folder};

use super::super::message::Message;
use super::super::state::Requiem;

impl Requiem {
    /// Select a request from collection
    pub fn handle_select_request(&mut self, path: Vec<usize>) -> Task<Message> {
        // Auto-save rename if in progress
        if self.renaming_item.is_some() {
            let _ = self.update(Message::ConfirmRename);
        }

        if !path.is_empty() {
            self.selected_collection = Some(path[0]);
        }
        self.context_menu = None;
        self.update(Message::OpenTab(path))
    }

    /// Add new request to collection
    pub fn handle_add_new_request(&mut self, parent_path: Vec<usize>) -> Task<Message> {
        // Auto-save rename if in progress
        if self.renaming_item.is_some() {
            let _ = self.update(Message::ConfirmRename);
        }

        let new_request = models::Request::default();
        let request_id = new_request.id;
        let request_name = new_request.name.clone();

        let new_tab = super::super::state::RequestTabItem {
            id: request_id,
            name: request_name.clone(),
            request_path: None,
            is_modified: false,
            is_new: true,
            draft_request: Some(new_request),
            parent_path: Some(parent_path.clone()),
        };

        self.open_tabs.push(new_tab);
        self.active_tab_index = Some(self.open_tabs.len() - 1);
        self.selected_request = None;
        self.selected_collection = Some(parent_path[0]);
        self.response = None;
        self.context_menu = None;

        Task::none()
    }

    /// Add new folder to collection
    pub fn handle_add_new_folder(&mut self, parent_path: Vec<usize>) -> Task<Message> {
        // Auto-save rename if in progress
        if self.renaming_item.is_some() {
            let _ = self.update(Message::ConfirmRename);
        }

        let folder_name = "New Folder".to_string();
        let new_folder = CollectionItem::Folder(Folder {
            id: Uuid::new_v4(),
            name: folder_name.clone(),
            items: vec![],
            expanded: true,
        });

        let collection_idx = parent_path[0];
        let mut new_folder_path = parent_path.clone();

        if parent_path.len() == 1 {
            if let Some(collection) = self.collections.get_mut(collection_idx) {
                collection.items.push(new_folder);
                new_folder_path.push(collection.items.len() - 1);
            }
        } else {
            if let Some(CollectionItem::Folder(folder)) = self.get_item_by_path_mut(&parent_path) {
                folder.items.push(new_folder);
                new_folder_path.push(folder.items.len() - 1);
            }
        }

        if let Err(e) = self.save_collection(collection_idx) {
            error!("Failed to save collection after adding folder: {}", e);
        }

        self.context_menu = None;

        // Set renaming state to auto-edit the new folder name
        self.renaming_item = Some((new_folder_path, folder_name.clone(), folder_name));

        // Focus and select the rename input
        Task::batch([
            focus(self.rename_input_id.clone()),
            select_all(self.rename_input_id.clone()),
        ])
    }

    /// Delete item from collection
    pub fn handle_delete_item(&mut self, path: Vec<usize>) -> Task<Message> {
        // Auto-save rename if in progress
        if self.renaming_item.is_some() {
            let _ = self.update(Message::ConfirmRename);
        }

        if path.is_empty() {
            return Task::none();
        }

        // Delete entire collection
        if path.len() == 1 {
            let collection_idx = path[0];
            if collection_idx < self.collections.len() {
                let collection = self.collections.remove(collection_idx);

                // Delete the collection file
                if let Err(e) = self.delete_collection_file(&collection.id) {
                    error!("Failed to delete collection file: {}", e);
                }

                // Clear selection if it was in this collection
                if let Some(selected_path) = &self.selected_request {
                    if selected_path.first() == Some(&collection_idx) {
                        self.selected_request = None;
                        self.response = None;
                    }
                }

                // Update selected_collection index if needed
                if self.selected_collection == Some(collection_idx) {
                    self.selected_collection = None;
                } else if let Some(selected) = self.selected_collection {
                    if selected > collection_idx {
                        self.selected_collection = Some(selected - 1);
                    }
                }

                // Close tabs that belong to this collection
                self.open_tabs.retain(|tab| {
                    if let Some(tab_path) = &tab.request_path {
                        tab_path.first() != Some(&collection_idx)
                    } else if let Some(parent_path) = &tab.parent_path {
                        parent_path.first() != Some(&collection_idx)
                    } else {
                        true
                    }
                });

                // Update active tab index if needed
                if let Some(active_idx) = self.active_tab_index {
                    if active_idx >= self.open_tabs.len() {
                        self.active_tab_index = if self.open_tabs.is_empty() {
                            None
                        } else {
                            Some(self.open_tabs.len() - 1)
                        };
                    }
                }
            }

            self.context_menu = None;
            return Task::none();
        }

        // Delete item within collection
        let parent_path = &path[..path.len() - 1];
        let item_idx = path[path.len() - 1];
        let collection_idx = path[0];

        if parent_path.len() == 1 {
            if let Some(collection) = self.collections.get_mut(parent_path[0]) {
                if item_idx < collection.items.len() {
                    collection.items.remove(item_idx);

                    if self.selected_request.as_ref() == Some(&path) {
                        self.selected_request = None;
                        self.response = None;
                    }
                }
            }
        } else {
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

        if let Err(e) = self.save_collection(collection_idx) {
            error!("Failed to save collection after deleting item: {}", e);
        }

        self.context_menu = None;
        Task::none()
    }

    /// Rename item in collection
    pub fn handle_rename_item(&mut self, path: Vec<usize>, new_name: String) -> Task<Message> {
        let collection_idx = path[0];
        let renamed_id = if path.len() == 1 {
            if let Some(collection) = self.collections.get_mut(path[0]) {
                collection.name = new_name.clone();
                Some(collection.id)
            } else {
                None
            }
        } else {
            if let Some(item) = self.get_item_by_path_mut(&path) {
                match item {
                    CollectionItem::Request(req) => {
                        debug!(
                            "Renaming request: {} -> {}, ID: {}",
                            req.name, new_name, req.id
                        );
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

        // Update tab names if this request is open in any tabs
        if let Some(id) = renamed_id {
            debug!("Looking for tabs with ID: {}", id);
            debug!(
                "Open tabs: {:?}",
                self.open_tabs
                    .iter()
                    .map(|t| (t.id, &t.name))
                    .collect::<Vec<_>>()
            );

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

        if let Err(e) = self.save_collection(collection_idx) {
            error!("Failed to save collection after renaming: {}", e);
        }

        self.renaming_item = None;
        self.context_menu = None;
        Task::none()
    }

    /// Toggle folder/collection expansion
    pub fn handle_toggle_expanded(&mut self, path: Vec<usize>) -> Task<Message> {
        // Auto-save rename if in progress
        if self.renaming_item.is_some() {
            let _ = self.update(Message::ConfirmRename);
        }

        if path.len() == 1 {
            if let Some(collection) = self.collections.get_mut(path[0]) {
                collection.expanded = !collection.expanded;
            }
        } else {
            if let Some(CollectionItem::Folder(folder)) = self.get_item_by_path_mut(&path) {
                folder.expanded = !folder.expanded;
            }
        }
        Task::none()
    }

    /// Add new collection
    pub fn handle_add_new_collection(&mut self) -> Task<Message> {
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

        if let Err(e) = self.save_collection(new_coll_idx) {
            error!("Failed to save new collection: {}", e);
        }

        self.context_menu = None;
        self.renaming_item = Some((vec![new_coll_idx], default_name.clone(), default_name));

        // Focus and select the rename input
        Task::batch([
            focus(self.rename_input_id.clone()),
            select_all(self.rename_input_id.clone()),
        ])
    }

    /// Start renaming an item
    pub fn handle_start_rename(&mut self, path: Vec<usize>) -> Task<Message> {
        let name = if path.len() == 1 {
            self.collections.get(path[0]).map(|c| c.name.clone())
        } else {
            self.get_item_by_path(&path).map(|item| match item {
                CollectionItem::Request(req) => req.name.clone(),
                CollectionItem::Folder(folder) => folder.name.clone(),
            })
        }
        .unwrap_or_default();

        self.renaming_item = Some((path, name.clone(), name));
        self.context_menu = None;

        Task::batch([
            focus(self.rename_input_id.clone()),
            select_all(self.rename_input_id.clone()),
        ])
    }

    /// Update renaming text
    pub fn handle_update_renaming_text(&mut self, new_text: String) -> Task<Message> {
        if let Some((path, original_name, _)) = &self.renaming_item {
            self.renaming_item = Some((path.clone(), original_name.clone(), new_text));
        }
        Task::none()
    }

    /// Confirm rename operation
    pub fn handle_confirm_rename(&mut self) -> Task<Message> {
        if let Some((path, original_name, current_name)) = self.renaming_item.clone() {
            self.renaming_item = None;
            if current_name != original_name && !current_name.trim().is_empty() {
                return self.update(Message::RenameItem(path, current_name));
            }
        }
        Task::none()
    }

    /// Cancel rename operation
    pub fn handle_cancel_rename(&mut self) -> Task<Message> {
        self.renaming_item = None;
        Task::none()
    }

    /// Save current request
    pub fn handle_save_request(&mut self) -> Task<Message> {
        let mut collection_idx_to_save = None;
        let mut newly_saved_path = None;

        if let Some(active_idx) = self.active_tab_index {
            if let Some(tab) = self.open_tabs.get_mut(active_idx) {
                if tab.is_new {
                    if let (Some(draft_request), Some(parent_path)) =
                        (tab.draft_request.clone(), tab.parent_path.clone())
                    {
                        let new_item = CollectionItem::Request(draft_request.clone());

                        let new_path = if parent_path.len() == 1 {
                            let collection_idx = parent_path[0];
                            if let Some(collection) = self.collections.get_mut(collection_idx) {
                                collection.items.push(new_item);
                                let new_idx = collection.items.len() - 1;
                                collection_idx_to_save = Some(collection_idx);
                                vec![collection_idx, new_idx]
                            } else {
                                vec![]
                            }
                        } else {
                            let collection_idx = parent_path[0];
                            if let Some(collection) = self.collections.get_mut(collection_idx) {
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
                                        if let Some(CollectionItem::Folder(folder)) =
                                            items.get_mut(idx)
                                        {
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
                            tab.request_path = Some(new_path.clone());
                            tab.is_new = false;
                            tab.draft_request = None;
                            tab.parent_path = None;

                            self.selected_request = Some(new_path.clone());
                            newly_saved_path = Some(new_path);

                            info!("New request saved to parent path: {:?}", parent_path);
                        }
                    }
                } else if let Some(request_path) = &tab.request_path {
                    collection_idx_to_save = Some(request_path[0]);
                }
            }
        }

        if let Some(idx) = collection_idx_to_save {
            if let Err(e) = self.save_collection(idx) {
                error!("Failed to save collection after saving request: {}", e);
            }
        }

        let toast = crate::ui::toast::Toast::success("Request saved successfully");
        self.toast = Some(toast.clone());

        let duration = toast.duration;
        let hide_toast_task = Task::perform(
            async move {
                tokio::time::sleep(duration).await;
            },
            |_| Message::HideToast,
        );

        if let Some(path) = newly_saved_path {
            Task::batch([hide_toast_task, Task::done(Message::StartRename(path))])
        } else {
            hide_toast_task
        }
    }
}
