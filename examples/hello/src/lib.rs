use oak::prelude::*;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::render("Hello World!").mount_to_body()
}
