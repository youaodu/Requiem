mod collection;
mod key_value;
mod request;
mod tabs;
mod ui;

use iced::Task;

use super::message::Message;
use super::state::Requiem;

impl Requiem {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // ============ Request Operations ============
            Message::MethodSelected(method) => self.handle_method_selected(method),
            Message::UrlChanged(url) => self.handle_url_changed(url),
            Message::SendRequest => self.handle_send_request(),
            Message::RequestSent(result) => self.handle_request_sent(result),
            Message::BodyChanged(body) => self.handle_body_changed(body),
            Message::BodyFormatChanged(format) => self.handle_body_format_changed(format),
            Message::FormDataKeyChanged(idx, key) => self.handle_form_data_key_changed(idx, key),
            Message::FormDataValueChanged(idx, value) => self.handle_form_data_value_changed(idx, value),
            Message::AddFormDataField => self.handle_add_form_data_field(),
            Message::RemoveFormDataField(idx) => self.handle_remove_form_data_field(idx),
            Message::RequestBodyAction(action) => self.handle_request_body_action(action),
            Message::ResponseBodyAction(action) => self.handle_response_body_action(action),
            Message::CopyResponseBody => self.handle_copy_response_body(),

            // ============ Key-Value Operations ============
            Message::HeaderKeyChanged(idx, key) => self.handle_header_key_changed(idx, key),
            Message::HeaderValueChanged(idx, value) => self.handle_header_value_changed(idx, value),
            Message::AddHeader => self.handle_add_header(),
            Message::RemoveHeader(idx) => self.handle_remove_header(idx),

            Message::ParamKeyChanged(idx, key) => self.handle_param_key_changed(idx, key),
            Message::ParamValueChanged(idx, value) => self.handle_param_value_changed(idx, value),
            Message::AddParam => self.handle_add_param(),
            Message::RemoveParam(idx) => self.handle_remove_param(idx),

            Message::CookieKeyChanged(idx, key) => self.handle_cookie_key_changed(idx, key),
            Message::CookieValueChanged(idx, value) => self.handle_cookie_value_changed(idx, value),
            Message::AddCookie => self.handle_add_cookie(),
            Message::RemoveCookie(idx) => self.handle_remove_cookie(idx),

            Message::AuthKeyChanged(idx, key) => self.handle_auth_key_changed(idx, key),
            Message::AuthValueChanged(idx, value) => self.handle_auth_value_changed(idx, value),
            Message::AddAuthField => self.handle_add_auth_field(),
            Message::RemoveAuthField(idx) => self.handle_remove_auth_field(idx),

            // ============ Collection Operations ============
            Message::SelectRequest(path) => self.handle_select_request(path),
            Message::AddNewRequest(parent_path) => self.handle_add_new_request(parent_path),
            Message::AddNewFolder(parent_path) => self.handle_add_new_folder(parent_path),
            Message::DeleteItem(path) => self.handle_delete_item(path),
            Message::RenameItem(path, new_name) => self.handle_rename_item(path, new_name),
            Message::ToggleExpanded(path) => self.handle_toggle_expanded(path),
            Message::AddNewCollection => self.handle_add_new_collection(),
            Message::StartRename(path) => self.handle_start_rename(path),
            Message::UpdateRenamingText(new_text) => self.handle_update_renaming_text(new_text),
            Message::ConfirmRename => self.handle_confirm_rename(),
            Message::CancelRename => self.handle_cancel_rename(),
            Message::SaveRequest => self.handle_save_request(),

            // ============ Tab Operations ============
            Message::TabSelected(tab) => self.handle_tab_selected(tab),
            Message::ResponseTabSelected(tab) => self.handle_response_tab_selected(tab),
            Message::OpenTab(path) => self.handle_open_tab(path),
            Message::CloseTab(index) => self.handle_close_tab(index),
            Message::SwitchTab(index) => self.handle_switch_tab(index),
            Message::TabDragStart(index, offset_x) => self.handle_tab_drag_start(index, offset_x),
            Message::TabDragMove(x) => self.handle_tab_drag_move(x),
            Message::TabDragEnd => self.handle_tab_drag_end(),
            Message::ReorderTabs(from_index, to_index) => self.handle_reorder_tabs(from_index, to_index),
            Message::MoveActiveTabLeft => self.handle_move_active_tab_left(),
            Message::MoveActiveTabRight => self.handle_move_active_tab_right(),
            Message::TabPressStart(index, x) => self.handle_tab_press_start(index, x),

            // ============ UI State ============
            Message::EnvironmentOptionSelected(option) => self.handle_environment_option_selected(option),
            Message::BodyViewModeSelected(mode) => self.handle_body_view_mode_selected(mode),
            Message::ShowContextMenu(path, x, y, target) => self.handle_show_context_menu(path, x, y, target),
            Message::HideContextMenu => self.handle_hide_context_menu(),
            Message::ShowToast(toast) => self.handle_show_toast(toast),
            Message::HideToast => self.handle_hide_toast(),
            Message::ShowEnvironmentDialog => self.handle_show_environment_dialog(),
            Message::CloseEnvironmentDialog => self.handle_close_environment_dialog(),
            Message::ShowSettingsDialog => self.handle_show_settings_dialog(),
            Message::CloseSettingsDialog => self.handle_close_settings_dialog(),
            Message::SaveDirectoryChanged(path) => self.handle_save_directory_changed(path),
            Message::BrowseSaveDirectory => self.handle_browse_save_directory(),
            Message::LanguageChanged(language) => self.handle_language_changed(language),
            Message::MouseMoved(x, y) => self.handle_mouse_moved(x, y),
            Message::SearchChanged(query) => self.handle_search_changed(query),
        }
    }
}
