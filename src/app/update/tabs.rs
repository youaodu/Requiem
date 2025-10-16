use iced::Task;
use tracing::info;
use uuid::Uuid;

use crate::models::{self, CollectionItem};

use super::super::message::Message;
use super::super::state::{DragState, Requiem, TabPressState};

impl Requiem {
    /// Switch to a specific tab
    pub fn handle_tab_selected(&mut self, tab: models::RequestTab) -> Task<Message> {
        self.active_tab = tab;

        // When switching to Body tab, sync request body to text editor content
        if tab == models::RequestTab::Body {
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

        Task::none()
    }

    /// Switch response tab
    pub fn handle_response_tab_selected(&mut self, tab: models::ResponseTab) -> Task<Message> {
        self.active_response_tab = tab;
        Task::none()
    }

    /// Open a request in a new or existing tab
    pub fn handle_open_tab(&mut self, path: Vec<usize>) -> Task<Message> {
        if let Some(existing_idx) = self.open_tabs.iter().position(|tab| tab.request_path.as_ref() == Some(&path)) {
            self.active_tab_index = Some(existing_idx);
            self.selected_request = Some(path);
        } else {
            let (name, id) = if let Some(item) = self.get_item_by_path(&path) {
                match item {
                    CollectionItem::Request(req) => {
                        tracing::debug!("Opening tab for request: name={}, id={}", req.name, req.id);
                        (req.name.clone(), req.id)
                    }
                    _ => ("Unknown".to_string(), Uuid::new_v4()),
                }
            } else {
                ("Unknown".to_string(), Uuid::new_v4())
            };

            let new_tab = super::super::state::RequestTabItem {
                id,
                name: name.clone(),
                request_path: Some(path.clone()),
                is_modified: false,
                is_new: false,
                draft_request: None,
                parent_path: None,
            };

            tracing::debug!("Created tab with id={}, name={}", id, name);
            self.open_tabs.push(new_tab);
            self.active_tab_index = Some(self.open_tabs.len() - 1);
            self.selected_request = Some(path);
        }
        self.response = None;
        Task::none()
    }

    /// Close a tab
    pub fn handle_close_tab(&mut self, index: usize) -> Task<Message> {
        info!("=== CLOSE TAB CALLED: index={} ===", index);

        // Auto-save rename if in progress
        if self.renaming_item.is_some() {
            let _ = self.update(Message::ConfirmRename);
        }

        if index < self.open_tabs.len() {
            self.open_tabs.remove(index);

            if self.open_tabs.is_empty() {
                self.active_tab_index = None;
                self.selected_request = None;
                self.response = None;
            } else if let Some(active_idx) = self.active_tab_index {
                if active_idx >= index {
                    self.active_tab_index = Some(active_idx.saturating_sub(1).min(self.open_tabs.len() - 1));
                }
                if let Some(new_active_idx) = self.active_tab_index {
                    if let Some(tab) = self.open_tabs.get(new_active_idx) {
                        self.selected_request = tab.request_path.clone();
                    }
                }
            }
        }
        Task::none()
    }

    /// Switch to a different tab
    pub fn handle_switch_tab(&mut self, index: usize) -> Task<Message> {
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

    /// Start tab drag operation
    pub fn handle_tab_drag_start(&mut self, index: usize, _offset_x: f32) -> Task<Message> {
        info!("=== TAB DRAG START CALLED: index={} ===", index);
        if index < self.open_tabs.len() {
            let initial_x = self.mouse_position.0;
            info!("Tab drag started: index={}, initial_x={}", index, initial_x);
            self.drag_state = Some(DragState {
                dragging_tab_index: index,
                hover_index: Some(index),
                initial_mouse_x: initial_x,
                current_mouse_x: initial_x,
            });
        }
        Task::none()
    }

    /// Handle tab drag movement
    pub fn handle_tab_drag_move(&mut self, x: f32) -> Task<Message> {
        if let Some(drag_state) = &mut self.drag_state {
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

    /// End tab drag operation
    pub fn handle_tab_drag_end(&mut self) -> Task<Message> {
        info!("=== TAB DRAG END CALLED (mouse_pos={}, {}) ===", self.mouse_position.0, self.mouse_position.1);

        // Handle tab press state (click without drag)
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

            if x_movement.abs() < 20.0 && duration.as_millis() < 300 {
                info!("Activating tab {} (click detected)", press_state.tab_index);
                self.active_tab_index = Some(press_state.tab_index);
                if let Some(tab) = self.open_tabs.get(press_state.tab_index) {
                    self.selected_request = tab.request_path.clone();
                }
            }
        }

        // Handle drag operation
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

    /// Reorder tabs
    pub fn handle_reorder_tabs(&mut self, from_index: usize, to_index: usize) -> Task<Message> {
        if from_index < self.open_tabs.len() && to_index < self.open_tabs.len() {
            let tab = self.open_tabs.remove(from_index);
            self.open_tabs.insert(to_index, tab);

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

    /// Move active tab left
    pub fn handle_move_active_tab_left(&mut self) -> Task<Message> {
        if let Some(active_idx) = self.active_tab_index {
            if active_idx > 0 {
                return self.update(Message::ReorderTabs(active_idx, active_idx - 1));
            }
        }
        Task::none()
    }

    /// Move active tab right
    pub fn handle_move_active_tab_right(&mut self) -> Task<Message> {
        if let Some(active_idx) = self.active_tab_index {
            if active_idx < self.open_tabs.len() - 1 {
                return self.update(Message::ReorderTabs(active_idx, active_idx + 1));
            }
        }
        Task::none()
    }

    /// Handle tab press start
    pub fn handle_tab_press_start(&mut self, index: usize, _x: f32) -> Task<Message> {
        let initial_x = self.mouse_position.0;
        let press_time = std::time::Instant::now();
        self.tab_press_state = Some(TabPressState {
            tab_index: index,
            press_time,
            initial_x,
            last_x: initial_x,
            delta_x: 0.0,
        });
        info!("=== TAB PRESS START: index={}, initial_x={}, time={:?} ===", index, initial_x, press_time);
        Task::none()
    }
}
