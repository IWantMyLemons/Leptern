use leptos::prelude::*;

use crate::state::recent_files::RecentFiles;

#[component]
pub fn RecentFile(recent_files: RecentFiles, filename: String, index: usize) -> impl IntoView {
    let on_click = move |_| {
        recent_files.remove(index);
    };
    view! { <li class="recent-file">{filename}<button on:click=on_click>"X"</button></li> }
}
