use leptos::prelude::*;
use leptos::serde_json;

#[derive(Clone, Debug)]
/// Manages and syncs recent filenames with LocalStorage
pub struct RecentFiles {
    pub files: RwSignal<Vec<String>>,
}

impl RecentFiles {
    /// Loads files from LocalStorage
    pub fn new() -> Self {
        let files = load_from_storage();
        Self {
            files: RwSignal::new(files),
        }
    }

    /// Adds the filename to recent files and LocalStorage
    pub fn add(&self, file: String) {
        self.files.update(move |files| {
            files.push(file);
            save_to_storage(files);
        });
    }

    /// Removes a filename by it's index
    pub fn remove(&self, index: usize) {
        self.files.update(move |files| {
            files.remove(index);
            save_to_storage(files);
        });
    }
}

/// Loads "recent_files" from LocalStorage
fn load_from_storage() -> Vec<String> {
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
fn save_to_storage(files: &[String]) {
    let recent_str = serde_json::to_string(files).unwrap_or("[]".to_string());
    let storage = window().local_storage().unwrap().unwrap();
    storage.set_item("recent_files", &recent_str).unwrap();
}
