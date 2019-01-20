use oak::{
    cmd,
    html::{button, div, events::on_click, text, Html},
    Cmd,
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
pub struct Model {
    counter: i32,
}

fn init() -> Model {
    Model { counter: 0 }
}

fn update(msg: &Msg, model: &mut Model) -> Box<Cmd<Msg>> {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
    }
    Box::new(cmd::None)
}

fn view(model: &Model) -> Html<Msg> {
    div(
        &[],
        &[
            button(&[on_click(Msg::Increment)], &[text("+")]),
            div(&[], &[text(&model.counter.to_string())]),
            button(&[on_click(Msg::Decrement)], &[text("-")]),
        ],
    )
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    oak::browser::sandbox(init(), view, update)
}
