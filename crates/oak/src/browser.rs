use wasm_bindgen::prelude::JsValue;

pub struct Resources {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
}

impl Resources {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        Ok(Self { window, document })
    }
}