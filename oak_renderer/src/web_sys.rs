use super::Renderer;
use wasm_bindgen::JsValue;

pub struct WebSysRenderer {
    document: web_sys::Document;
}

impl Renderer for WebSysRenderer {
    type NodeType = web_sys::Node;
    type Error = JsValue;
    fn create_node(&self, node: oak_vdom::Node) -> Result<Self::NodeType, Self::Error> {

    }
}