use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlTemplateElement;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let compiler_template = document
        .create_element("template")
        .unwrap()
        .unchecked_into::<HtmlTemplateElement>();
    let tree_walker = document.create_tree_walker(&document)?;
    let template = stage0::compile_str(
        &compiler_template,
        &tree_walker,
        "<div #t1><h1 #t2>Test</h1></div>",
    )
    .unwrap();
    document.body().unwrap().append_child(&template.node)?;
    web_sys::console::log_1(&template.node);
    let refs = template.collect(&tree_walker)?;
    let t1 = &refs["t1"];
    web_sys::console::log_1(&t1);
    let t2 = &refs["t2"];
    web_sys::console::log_1(&t2);
    Ok(())
}
