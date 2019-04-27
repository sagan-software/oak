use oak::prelude::*;

#[wasm_bindgen]
pub fn main() {
    oak::run("body", html("Hello World!"))
}
