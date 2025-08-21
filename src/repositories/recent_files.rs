use leptos::prelude::*;
use leptos::serde_json;

/// Loads "recent_files" from LocalStorage
pub fn load_recents_from_storage() -> Vec<String> {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("recent_files").ok())
        .flatten()
        .and_then(|json| serde_json::from_str::<Vec<String>>(&json).ok())
        .unwrap_or_default()
}

/// Saves `files` to "recent_files" in LocalStorage
pub fn save_recents_to_storage(files: &[String]) {
    let recent_str = serde_json::to_string(files).unwrap_or("[]".to_string());
    let storage = window().local_storage().unwrap().unwrap();
    storage.set_item("recent_files", &recent_str).unwrap();
}
