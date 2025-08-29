use leptos::prelude::*;

use crate::{
    components::{pdf_renderer::PdfRenderer, text_renderer::PlaintextRenderer},
    repositories::file_store::load_file_from_idb,
    state::idb_context::IdbContext,
};

#[component]
pub fn FileReader(filename: String) -> impl IntoView {
    let db_context = use_context::<IdbContext>().unwrap();
    let db = db_context.db;
    let (current_data, set_current_data) = signal_local(None);

    let filename_0 = filename.clone();
    Effect::new(move || {
        if let Some(db) = db.get() {
            load_file_from_idb(db, filename_0.clone(), set_current_data);
        }
    });

    view! {
        {move || match current_data.get() {
            Some(data) if filename.ends_with(".pdf") => {
                view! { <PdfRenderer data=data /> }.into_any()
            }
            Some(data) => view! { <PlaintextRenderer data=data /> }.into_any(),
            None => view! { "nothing..." }.into_any(),
        }}
    }
}
