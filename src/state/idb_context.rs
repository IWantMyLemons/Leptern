use leptos::prelude::*;
use web_sys::IdbDatabase;

#[derive(Clone, Debug)]
pub struct IdbContext {
    pub db: RwSignal<Option<IdbDatabase>, LocalStorage>,
}

impl IdbContext {
    pub fn new() -> Self {
        Self {
            db: RwSignal::new_local(None),
        }
    }
}
