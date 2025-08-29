use leptos::prelude::*;

use crate::state::{current_file::CurrentFile, recent_files::RecentFiles};

#[component]
pub fn RecentFile(recent_files: RecentFiles, filename: String, index: usize) -> impl IntoView {
    let current_file = use_context::<CurrentFile>().unwrap();
    let on_click = move |_| {
        recent_files.remove(index);
    };
    let fileclone = filename.clone();
    view! {
        <li class="recent-file">
            <button on:click=move |_| current_file.set(fileclone.clone())>{filename}</button>
            <button on:click=on_click>"X"</button>
        </li>
    }
}
