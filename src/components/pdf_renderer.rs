use crate::bindings::pdfjs::{fetch_document_with_opts, PdfOpts, PdfRenderContext, PdfViewportOpts};
use leptos::{html, prelude::*, reactive::spawn_local};
use web_sys::{console, js_sys::ArrayBuffer};

#[component]
pub fn PdfRenderer(data: ArrayBuffer) -> impl IntoView {
    let canvas_element = NodeRef::<html::Canvas>::new();
    canvas_element.on_load(move |canvas| {
        spawn_local(async move {
            let opts = PdfOpts::from_data(&data);
            let document = fetch_document_with_opts(opts).await.unwrap();
            let first_page = document.fetch_page(1).await.unwrap();

            let canvas_context = canvas.get_context("2d").unwrap().unwrap();
            let device_pixel_ratio = window().device_pixel_ratio();
            let viewport_opts = PdfViewportOpts::from_scale(device_pixel_ratio);
            let viewport = first_page.get_viewport(viewport_opts);

            let render_context = PdfRenderContext::new(&canvas_context, &viewport);

            canvas.set_width(viewport.width().round() as u32);
            canvas.set_height(viewport.height().round() as u32);

            console::log_1(&render_context);
            first_page.render(render_context);
        });
    });
    view! { <canvas class="pdf-renderer" node_ref=canvas_element></canvas> }
}
