use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;
use web_sys::{Event, IdbDatabase, IdbRequest};

const DATABASE_VERSION: u32 = 2;

/// Attaches a database onto a signal, if a failure happens it will log to terminal
/// since i don't know how to handle errors from callbacks ._.
pub fn attach_db(signal: RwSignal<Option<IdbDatabase>, LocalStorage>) {
    let request = window()
        .indexed_db()
        .ok()
        .flatten()
        .and_then(|factory| factory.open_with_u32("FileDatabase", DATABASE_VERSION).ok())
        .unwrap();

    let on_success = Closure::<dyn FnMut(_)>::new(move |ev: Event| {
        let db = event_target::<IdbRequest>(&ev)
            .result()
            .unwrap()
            .dyn_into::<IdbDatabase>()
            .unwrap();
        signal.set(Some(db));
    });

    let on_upgrade_needed = Closure::<dyn FnMut(_)>::new(move |ev: Event| {
        let db = event_target::<IdbRequest>(&ev)
            .result()
            .unwrap()
            .dyn_into::<IdbDatabase>()
            .unwrap();
        let files_store = db.create_object_store("files").unwrap();
        files_store.create_index_with_str("name", "name").unwrap();
        log::info!("upgraded database! :3");
    });

    request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    request.set_onupgradeneeded(Some(on_upgrade_needed.as_ref().unchecked_ref()));
    on_success.forget();
    on_upgrade_needed.forget();
}
