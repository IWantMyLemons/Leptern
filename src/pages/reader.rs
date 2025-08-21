use leptos::prelude::*;

use crate::{
    components::{file_browser::FileBrowser, file_reader::FileReader, sidebar::Sidebar},
    state::current_file::CurrentFile,
};

/// Default Page
#[component]
pub fn Reader() -> impl IntoView {
    let current_file = use_context::<CurrentFile>().unwrap();

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <div class="reader">
                <Sidebar />
                <div class="pdf-reader">
                    {move || match current_file.filename.get() {
                        Some(filename) => view! { <FileReader filename=filename /> }.into_any(),
                        None => view! { <FileBrowser /> }.into_any(),
                    }}
                </div>
            </div>
        </ErrorBoundary>
    }
}
