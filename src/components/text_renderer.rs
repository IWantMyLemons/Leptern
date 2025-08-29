use leptos::prelude::*;
use web_sys::{js_sys::ArrayBuffer, TextDecoder};

#[component]
pub fn PlaintextRenderer(data: ArrayBuffer) -> impl IntoView {
    let decoder = TextDecoder::new().unwrap();
    let text = decoder.clone().decode_with_buffer_source(&data).unwrap();

    view! { {text} }
}
