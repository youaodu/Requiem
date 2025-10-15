use serde::{Deserialize, Serialize};

use super::key_value::KeyValue;

/// Body Format Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyFormat {
    None,
    FormData,
    FormUrlEncoded,
    Json,
    Xml,
    Text,
    Binary,
}

impl BodyFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            BodyFormat::None => "none",
            BodyFormat::FormData => "form-data",
            BodyFormat::FormUrlEncoded => "x-www-form-urlencoded",
            BodyFormat::Json => "JSON",
            BodyFormat::Xml => "XML",
            BodyFormat::Text => "Text",
            BodyFormat::Binary => "Binary",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            BodyFormat::None,
            BodyFormat::FormData,
            BodyFormat::FormUrlEncoded,
            BodyFormat::Json,
            BodyFormat::Xml,
            BodyFormat::Text,
            BodyFormat::Binary,
        ]
    }
}

/// Request Body Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyType {
    None,
    Json(String),
    FormUrlEncoded(Vec<KeyValue>),
    FormData(Vec<KeyValue>),
    Xml(String),
    Text(String),
    Binary(Vec<u8>),
}

impl Default for BodyType {
    fn default() -> Self {
        BodyType::None
    }
}

impl BodyType {
    pub fn format(&self) -> BodyFormat {
        match self {
            BodyType::None => BodyFormat::None,
            BodyType::Json(_) => BodyFormat::Json,
            BodyType::FormUrlEncoded(_) => BodyFormat::FormUrlEncoded,
            BodyType::FormData(_) => BodyFormat::FormData,
            BodyType::Xml(_) => BodyFormat::Xml,
            BodyType::Text(_) => BodyFormat::Text,
            BodyType::Binary(_) => BodyFormat::Binary,
        }
    }
}
