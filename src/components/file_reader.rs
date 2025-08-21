use leptos::prelude::*;

#[component]
pub fn FileReader(filename: String) -> impl IntoView {
    view! {
        "Imagine you're reading "
        {filename}
        " ..."
        <canvas></canvas>
    }
}
