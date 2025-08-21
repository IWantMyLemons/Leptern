use leptos::prelude::*;

use crate::components::{file_browser::FileBrowser, sidebar::Sidebar};

/// Default Page
#[component]
pub fn Reader() -> impl IntoView {
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
                    <FileBrowser />
                </div>
            </div>
        </ErrorBoundary>
    }
}
