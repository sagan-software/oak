use oak::prelude::*;

fn view(name: &str) -> Html {
    html! {
        <h1>Hello, { name }!</h1>
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    App::with_state("World")
        .with_view(view)
        .start("body")
}