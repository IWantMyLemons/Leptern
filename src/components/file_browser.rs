use leptos::prelude::*;
use leptos::web_sys::{Event, HtmlInputElement};

use crate::repositories::file_store::save_file_to_idb;
use crate::state::current_file::CurrentFile;
use crate::state::idb_context::IdbContext;
use crate::state::recent_files::RecentFiles;

#[component]
pub fn FileBrowser() -> impl IntoView {
    let recent = use_context::<RecentFiles>().unwrap();
    let current_file = use_context::<CurrentFile>().unwrap();
    let db_context = use_context::<IdbContext>().unwrap();
    let db = move || db_context.db.get().unwrap();

    let on_file_change = move |ev: Event| {
        let input = event_target::<HtmlInputElement>(&ev);
        if let Some(file) = input.files().and_then(|files| files.get(0)) {
            recent.add(file.name());
            current_file.set(file.name());
            save_file_to_idb(db(), file);
        }
    };
    view! { <input type="file" on:change=on_file_change /> }
}
