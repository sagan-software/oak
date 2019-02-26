use oak::prelude::*;

pub type Model = Time;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    Tick,
}

fn init() -> (Model, impl Cmd<Msg>) {
    (Time::now(), platform::None)
}

fn update(msg: &Msg, model: &mut Model) -> impl Cmd<Msg> {
    match msg {
        Msg::Tick => *model = Time::now(),
    }
    platform::None
}

fn view(model: &Model) -> Html<Msg> {
    div(
        [],
        [
            p([], ["The current time is:"]),
            p([], [strong([], [model.to_string()])]),
        ],
    )
}

fn subscriptions(model: &Model) -> impl Sub<Msg> {
    platform::None
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    oak::element(init, view, update, subscriptions).init("#app")
}
