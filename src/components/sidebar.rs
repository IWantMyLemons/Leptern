use leptos::prelude::*;

use crate::state::recent_files::RecentFiles;

#[component]
pub fn Sidebar() -> impl IntoView {
    let recent = use_context::<RecentFiles>().unwrap();

    view! {
        <div class="sidebar">
            <ul class="recents">
                {move || {
                    recent
                        .files
                        .get()
                        .into_iter()
                        .map(|filename| view! { <li>{filename}</li> })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}
