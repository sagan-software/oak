use oak::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    app::stateless(h1().push("Hello World!")).mount("body")
}
