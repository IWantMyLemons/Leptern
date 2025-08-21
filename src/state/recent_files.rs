use leptos::prelude::*;
use leptos::serde_json;

#[derive(Clone, Debug)]
pub struct RecentFiles {
    pub files: RwSignal<Vec<String>>,
}

impl RecentFiles {
    pub fn new() -> Self {
        let files = load_from_storage();
        Self {
            files: RwSignal::new(files),
        }
    }

    pub fn add(&self, file: String) {
        let mut files = self.files.get();
        files.push(file);
        self.files.set(files.clone());
        save_to_storage(&files);
    }
}

fn load_from_storage() -> Vec<String> {
    let storage = window().local_storage().unwrap().unwrap();
    let recent_str = storage
        .get_item("recent_files")
        .unwrap()
        .unwrap_or("[]".to_string());
    serde_json::from_str::<Vec<String>>(&recent_str).unwrap_or_default()
}

fn save_to_storage(files: &Vec<String>) {
    let recent_str = serde_json::to_string(files).unwrap_or("[]".to_string());
    let storage = window().local_storage().unwrap().unwrap();
    storage.set_item("recent_files", &recent_str).unwrap();
}
