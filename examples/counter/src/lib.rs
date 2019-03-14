use oak::prelude::*;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::update(update).view(view).mount_to_body()
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

fn view(count: i32) -> HtmlElement<Msg> {
    div()
        .push(button().on(click(Msg::Decrement)).push("-"))
        .push(count)
        .push(button().on(click(Msg::Increment)).push("+"))
}
