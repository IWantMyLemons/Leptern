use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{self, Object, Promise};

// Raw PdfJs functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = pdfjsLib, js_name = getDocument, catch)]
    pub fn get_document_with_opts(opts: PdfOpts) -> Result<PdfLoadTask, JsValue>;
}

// PdfOpts
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type PdfOpts;
}

impl PdfOpts {
    pub fn from_data(data: &JsValue) -> Self {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("data"), data).unwrap();

        obj.unchecked_into()
    }
}
// PdfDocument
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type PdfLoadTask;

    #[wasm_bindgen(method, getter)]
    pub fn promise(this: &PdfLoadTask) -> Promise;
}

pub async fn fetch_document_with_opts(opts: PdfOpts) -> Result<PdfDocument, JsValue> {
    let request = get_document_with_opts(opts)?;
    let future = JsFuture::from(request.promise());
    future.await.map(|success| success.unchecked_into::<PdfDocument>())
}

// PdfDocument
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type PdfDocument;

    // gets the page of a pdf, page_num is 1-indexed
    #[wasm_bindgen(method, js_name = getPage)]
    pub fn get_page(this: &PdfDocument, page_num: u32) -> Promise;
}

impl PdfDocument {
    pub async fn fetch_page(&self, page_num: u32) -> Result<PdfPage, JsValue> {
        let promise = self.get_page(page_num);
        let future = JsFuture::from(promise);
        future.await.map(|success| success.unchecked_into::<PdfPage>())
    }
}

// PdfPage
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type PdfPage;

    #[wasm_bindgen(method, js_name = getViewport)]
    pub fn get_viewport(this: &PdfPage, viewport_opts: PdfViewportOpts) -> PdfViewport;

    #[wasm_bindgen(method, js_name = render)]
    pub fn render(this: &PdfPage, context: PdfRenderContext) -> PdfRenderTask;
}

// PdfViewportOpts
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type PdfViewportOpts;
}

impl PdfViewportOpts {
    pub fn from_scale(scale: f64) -> Self {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("scale"), &JsValue::from_f64(scale)).unwrap();
        
        obj.unchecked_into()
    }
}

// PdfViewport
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type PdfViewport;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &PdfViewport) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &PdfViewport) -> f64;
}

// PdfRenderContext
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type PdfRenderContext;
}

impl PdfRenderContext {
    pub fn new(canvas_context: &Object, viewport: &PdfViewport) -> Self {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("canvasContext"), canvas_context).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("viewport"), viewport).unwrap();

        obj.unchecked_into()
    }
}

// PdfRenderTask
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type PdfRenderTask;

    #[wasm_bindgen(method, getter)]
    pub fn promise(this: &PdfRenderTask) -> Promise;
}