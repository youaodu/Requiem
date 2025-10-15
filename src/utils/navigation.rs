use crate::models::{Collection, CollectionItem};

/// Get an item by its path in the collection hierarchy
pub fn get_item_by_path<'a>(
    collections: &'a [Collection],
    path: &[usize],
) -> Option<&'a CollectionItem> {
    if path.is_empty() {
        return None;
    }

    let collection = collections.get(path[0])?;
    if path.len() == 1 {
        return None; // Path points to collection itself
    }

    let mut current_items = &collection.items;
    let mut item = current_items.get(path[1])?;

    for &idx in &path[2..] {
        if let CollectionItem::Folder(folder) = item {
            current_items = &folder.items;
            item = current_items.get(idx)?;
        } else {
            return None;
        }
    }

    Some(item)
}

/// Get a mutable item by its path in the collection hierarchy
pub fn get_item_by_path_mut<'a>(
    collections: &'a mut [Collection],
    path: &[usize],
) -> Option<&'a mut CollectionItem> {
    if path.is_empty() {
        return None;
    }

    let collection = collections.get_mut(path[0])?;
    if path.len() == 1 {
        return None; // Path points to collection itself
    }

    let mut current_items = &mut collection.items;
    let mut item = current_items.get_mut(path[1])?;

    for &idx in &path[2..] {
        if let CollectionItem::Folder(folder) = item {
            current_items = &mut folder.items;
            item = current_items.get_mut(idx)?;
        } else {
            return None;
        }
    }

    Some(item)
}

/// Get the name of an item by its path
pub fn get_item_name(collections: &[Collection], path: &[usize]) -> Option<String> {
    if path.is_empty() {
        return None;
    }

    if path.len() == 1 {
        return collections.get(path[0]).map(|c| c.name.clone());
    }

    let item = get_item_by_path(collections, path)?;
    match item {
        CollectionItem::Request(req) => Some(req.name.clone()),
        CollectionItem::Folder(folder) => Some(folder.name.clone()),
    }
}
