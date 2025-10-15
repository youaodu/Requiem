use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::request::Request;

/// Folder item that can contain requests or other folders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<CollectionItem>,
    pub expanded: bool,
}

/// Collection item - can be a request or a folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionItem {
    Request(Request),
    Folder(Folder),
}

/// Project/Collection structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<CollectionItem>,
    pub expanded: bool,
}
