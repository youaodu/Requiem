use iced::Task;

use crate::models;

use super::super::message::Message;
use super::super::state::Requiem;

impl Requiem {
    // ============ Headers ============

    pub fn handle_header_key_changed(&mut self, idx: usize, key: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(header) = request.headers.get_mut(idx) {
                header.key = key;
            }
        }
        Task::none()
    }

    pub fn handle_header_value_changed(&mut self, idx: usize, value: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(header) = request.headers.get_mut(idx) {
                header.value = value;
            }
        }
        Task::none()
    }

    pub fn handle_add_header(&mut self) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            request.headers.push(models::KeyValue::new("", ""));
        }
        Task::none()
    }

    pub fn handle_remove_header(&mut self, idx: usize) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if idx < request.headers.len() {
                request.headers.remove(idx);
            }
        }
        Task::none()
    }

    // ============ Query Parameters ============

    pub fn handle_param_key_changed(&mut self, idx: usize, key: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(param) = request.query_params.get_mut(idx) {
                param.key = key;
            }
        }
        Task::none()
    }

    pub fn handle_param_value_changed(&mut self, idx: usize, value: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(param) = request.query_params.get_mut(idx) {
                param.value = value;
            }
        }
        Task::none()
    }

    pub fn handle_add_param(&mut self) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            request.query_params.push(models::KeyValue::new("", ""));
        }
        Task::none()
    }

    pub fn handle_remove_param(&mut self, idx: usize) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if idx < request.query_params.len() {
                request.query_params.remove(idx);
            }
        }
        Task::none()
    }

    // ============ Cookies ============

    pub fn handle_cookie_key_changed(&mut self, idx: usize, key: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(cookie) = request.cookies.get_mut(idx) {
                cookie.key = key;
            }
        }
        Task::none()
    }

    pub fn handle_cookie_value_changed(&mut self, idx: usize, value: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(cookie) = request.cookies.get_mut(idx) {
                cookie.value = value;
            }
        }
        Task::none()
    }

    pub fn handle_add_cookie(&mut self) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            request.cookies.push(models::KeyValue::new("", ""));
        }
        Task::none()
    }

    pub fn handle_remove_cookie(&mut self, idx: usize) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if idx < request.cookies.len() {
                request.cookies.remove(idx);
            }
        }
        Task::none()
    }

    // ============ Authentication ============

    pub fn handle_auth_key_changed(&mut self, idx: usize, key: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(auth_field) = request.auth.get_mut(idx) {
                auth_field.key = key;
            }
        }
        Task::none()
    }

    pub fn handle_auth_value_changed(&mut self, idx: usize, value: String) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if let Some(auth_field) = request.auth.get_mut(idx) {
                auth_field.value = value;
            }
        }
        Task::none()
    }

    pub fn handle_add_auth_field(&mut self) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            request.auth.push(models::KeyValue::new("", ""));
        }
        Task::none()
    }

    pub fn handle_remove_auth_field(&mut self, idx: usize) -> Task<Message> {
        if let Some(request) = self.get_current_request_mut() {
            if idx < request.auth.len() {
                request.auth.remove(idx);
            }
        }
        Task::none()
    }
}
