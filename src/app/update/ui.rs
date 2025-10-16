use iced::Task;
use tracing::{error, info};

use crate::models::EnvironmentOption;

use super::super::message::Message;
use super::super::state::{ContextMenu, ContextMenuTarget, DragState, Requiem};

impl Requiem {
    /// Handle environment option selection
    pub fn handle_environment_option_selected(&mut self, option: EnvironmentOption) -> Task<Message> {
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
    pub fn handle_body_view_mode_selected(&mut self, mode: crate::models::BodyViewMode) -> Task<Message> {
        self.active_body_view_mode = mode;

        let formatted_body = match mode {
            crate::models::BodyViewMode::Raw => {
                self.raw_response_body.clone()
            }
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
            }
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

    /// Handle mouse movement
    pub fn handle_mouse_moved(&mut self, x: f32, y: f32) -> Task<Message> {
        self.mouse_position = (x, y);

        // Check if we should start dragging from tab press
        if let Some(press_state) = self.tab_press_state.as_ref() {
            let total_movement = x - press_state.initial_x;

            if self.drag_state.is_none() && total_movement.abs() > 20.0 {
                info!("=== STARTING DRAG: tab_index={}, movement={:.1}px ===", press_state.tab_index, total_movement);
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
                info!("Hover target changed: {:?} -> {} (movement={:.1}px, positions={})",
                      drag_state.hover_index, target_idx, movement, positions_moved);
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
}
