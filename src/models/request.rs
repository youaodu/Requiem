use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::body::BodyType;
use super::http_method::HttpMethod;
use super::key_value::KeyValue;

/// HTTP Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<KeyValue>,
    pub query_params: Vec<KeyValue>,
    pub cookies: Vec<KeyValue>,
    pub auth: Vec<KeyValue>,
    pub body: BodyType,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "New Request".to_string(),
            method: HttpMethod::GET,
            url: "https://api.example.com".to_string(),
            headers: vec![KeyValue::new("Content-Type", "application/json")],
            query_params: vec![],
            cookies: vec![],
            auth: vec![],
            body: BodyType::None,
        }
    }
}
