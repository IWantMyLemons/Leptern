use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::{Event, File, FileReader, HtmlInputElement};

use crate::state::current_file::CurrentFile;
use crate::state::recent_files::RecentFiles;

#[component]
pub fn FileBrowser() -> impl IntoView {
    let recent = use_context::<RecentFiles>().unwrap();
    let current_file = use_context::<CurrentFile>().unwrap();

    let (file_content, set_file_content) = signal(None);
    let on_file_change = move |ev: Event| {
        let input = event_target::<HtmlInputElement>(&ev);
        if let Some(file) = input.files().and_then(|files| files.get(0)) {
            recent.add(file.name());
            current_file.set(file.name());
            read_file_to_string(&file, set_file_content);
        }
    };
    view! { <input type="file" on:change=on_file_change /> }
}

fn read_file_to_string(file: &File, set_content: WriteSignal<Option<String>>) {
    let reader = FileReader::new().unwrap();
    let on_load_end = Closure::<dyn FnMut(_)>::new(move |ev: Event| {
        let reader = event_target::<FileReader>(&ev);
        if let Ok(result) = reader.result() {
            if let Some(text) = result.as_string() {
                set_content.set(Some(text));
            }
        }
    });
    reader.set_onloadend(Some(on_load_end.as_ref().unchecked_ref()));
    reader.read_as_text(file).unwrap();
    on_load_end.forget();
}
