use leptos::prelude::*;

use crate::repositories::recent_files::{load_recents_from_storage, save_recents_to_storage};

#[derive(Clone, Debug)]
/// Manages and syncs recent filenames with LocalStorage
pub struct RecentFiles {
    pub files: RwSignal<Vec<String>>,
}

impl RecentFiles {
    /// Loads files from LocalStorage
    pub fn new() -> Self {
        let files = load_recents_from_storage();
        Self {
            files: RwSignal::new(files),
        }
    }

    /// Adds the filename to recent files and LocalStorage
    pub fn add(&self, file: String) {
        self.files.update(move |files| {
            files.push(file);
            save_recents_to_storage(files);
        });
    }

    /// Removes a filename by it's index
    pub fn remove(&self, index: usize) {
        self.files.update(move |files| {
            files.remove(index);
            save_recents_to_storage(files);
        });
    }
}
