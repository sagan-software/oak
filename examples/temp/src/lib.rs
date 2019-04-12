use oak::prelude::*;
use serde::Serialize;
use wasm_typescript_definition::TypescriptDefinition;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::update(update).view(view).mount_to_body()
}

#[wasm_bindgen]
pub struct State {
    pub first: u32,
    pub second: u32,
}

#[wasm_bindgen]
pub enum Action {
    Add { name: u8 },
}

#[wasm_bindgen]
pub fn reducer(state: State, action: Action) -> State {
    match action {
        Action::Add => state,
    }
}

#[derive(Clone, Debug)]
enum Msg {
    Increment,
    Decrement,
}

fn update(count: i32, msg: Msg) -> i32 {
    match msg {
        Msg::Increment => count + 1,
        Msg::Decrement => count - 1,
    }
}

fn view(count: &i32) -> HtmlElement<Msg> {
    div()
        .push(button().set(on_click(Msg::Decrement)).push("-"))
        .push(count)
        .push(button().set(on_click(Msg::Increment)).push("+"))
}
