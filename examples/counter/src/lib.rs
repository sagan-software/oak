use oak::prelude::*;

pub type Model = i32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Increment,
    Decrement,
}

fn update(msg: &Msg, model: &mut Model) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::Decrement => *model -= 1,
    }
}

fn view(model: &Model) -> Html<Msg> {
    div(
        [],
        [
            button([on_click(Msg::Increment)], ["+"]),
            div([], [model]),
            button([on_click(Msg::Decrement)], ["-"]),
        ],
    )
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    oak::sandbox(0, view, update).init("#app")
}
