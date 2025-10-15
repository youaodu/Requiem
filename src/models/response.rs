use std::collections::HashMap;

/// HTTP Response
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub time_ms: u128,
    pub size_bytes: usize,
}

impl Response {
    pub fn new(
        status: u16,
        status_text: String,
        headers: HashMap<String, String>,
        body: String,
        time_ms: u128,
    ) -> Self {
        let size_bytes = body.len();
        Self {
            status,
            status_text,
            headers,
            body,
            time_ms,
            size_bytes,
        }
    }
}
