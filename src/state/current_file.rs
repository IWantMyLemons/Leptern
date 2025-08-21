use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct CurrentFile {
    pub filename: RwSignal<Option<String>>,
}

impl CurrentFile {
    pub fn new() -> Self {
        Self {
            filename: RwSignal::new(None),
        }
    }

    pub fn set(&self, filename: String) {
        self.filename.set(Some(filename));
    }
}
