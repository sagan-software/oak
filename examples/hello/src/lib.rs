use oak::prelude::*;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::view("Hello World!").mount_to_body()
}
