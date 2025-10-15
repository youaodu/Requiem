use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};

use crate::models::Collection;

/// Get the path to a collection file
pub fn get_collection_path(base_dir: &str, collection_id: &uuid::Uuid) -> PathBuf {
    Path::new(base_dir).join(format!("{}.json", collection_id))
}

/// Ensure the storage directory exists
pub fn ensure_storage_dir(base_dir: &str) -> Result<(), String> {
    let path = Path::new(base_dir);
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| format!("Failed to create storage directory: {}", e))?;
        info!("Created storage directory: {}", base_dir);
    }
    Ok(())
}

/// Save a collection to disk
pub fn save_collection(base_dir: &str, collection: &Collection) -> Result<(), String> {
    ensure_storage_dir(base_dir)?;

    let path = get_collection_path(base_dir, &collection.id);
    let json = serde_json::to_string_pretty(collection)
        .map_err(|e| format!("Failed to serialize collection: {}", e))?;

    fs::write(&path, json)
        .map_err(|e| format!("Failed to write collection file: {}", e))?;

    debug!("Saved collection '{}' to {:?}", collection.name, path);
    Ok(())
}

/// Load all collections from disk
pub fn load_collections(base_dir: &str) -> Result<Vec<Collection>, String> {
    let path = Path::new(base_dir);

    // If directory doesn't exist, return empty vector
    if !path.exists() {
        debug!("Storage directory does not exist: {}", base_dir);
        return Ok(vec![]);
    }

    let mut collections = Vec::new();

    let entries = fs::read_dir(path)
        .map_err(|e| format!("Failed to read storage directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        // Only process .json files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            match load_collection_from_file(&path) {
                Ok(collection) => {
                    debug!("Loaded collection '{}' from {:?}", collection.name, path);
                    collections.push(collection);
                }
                Err(e) => {
                    error!("Failed to load collection from {:?}: {}", path, e);
                }
            }
        }
    }

    info!("Loaded {} collections from {}", collections.len(), base_dir);
    Ok(collections)
}

/// Load a single collection from a file
fn load_collection_from_file(path: &Path) -> Result<Collection, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let collection: Collection = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to deserialize collection: {}", e))?;

    Ok(collection)
}

/// Delete a collection file from disk
pub fn delete_collection(base_dir: &str, collection_id: &uuid::Uuid) -> Result<(), String> {
    let path = get_collection_path(base_dir, collection_id);

    if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to delete collection file: {}", e))?;
        debug!("Deleted collection file: {:?}", path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_ensure_storage_dir() {
        let temp_dir = std::env::temp_dir().join("requiem_test");
        let temp_path = temp_dir.to_str().unwrap();

        // Clean up if exists
        let _ = fs::remove_dir_all(&temp_dir);

        // Test creating directory
        assert!(ensure_storage_dir(temp_path).is_ok());
        assert!(temp_dir.exists());

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_save_and_load_collection() {
        let temp_dir = std::env::temp_dir().join("requiem_test_save_load");
        let temp_path = temp_dir.to_str().unwrap();

        // Clean up if exists
        let _ = fs::remove_dir_all(&temp_dir);

        // Create test collection
        let collection = Collection {
            id: Uuid::new_v4(),
            name: "Test Collection".to_string(),
            items: vec![],
            expanded: true,
        };

        // Save collection
        assert!(save_collection(temp_path, &collection).is_ok());

        // Load collections
        let loaded = load_collections(temp_path).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].name, "Test Collection");
        assert_eq!(loaded[0].id, collection.id);

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
