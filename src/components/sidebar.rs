use leptos::prelude::*;

use crate::{components::recent_file::RecentFile, state::recent_files::RecentFiles};

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
                        .enumerate()
                        .map(|(index, filename)| {
                            view! {
                                <RecentFile
                                    filename=filename
                                    index=index
                                    recent_files=recent.clone()
                                />
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}
