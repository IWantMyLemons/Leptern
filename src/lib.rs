use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod bindings;
mod components;
mod pages;
mod repositories;
mod state;

// Top-Level pages
use crate::{
    pages::{not_found::NotFound, reader::Reader},
    repositories::idb_database::attach_db,
    state::{current_file::CurrentFile, idb_context::IdbContext, recent_files::RecentFiles},
};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    provide_context(RecentFiles::new());
    provide_context(CurrentFile::new());
    provide_context(IdbContext::new());

    // Attach database onto context as soon as possible
    {
        let db = use_context::<IdbContext>().unwrap();
        attach_db(db.db);
        log::info!("attached db :3");
    }

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Leptern" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| NotFound>
                <Route path=path!("/") view=Reader />
            </Routes>
        </Router>
    }
}
