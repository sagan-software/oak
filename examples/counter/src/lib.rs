use oak::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    app::with_state(Model::default())
        .with_view(view)
        .mount("body")
}

#[derive(Default)]
struct Model {
    count: i16,
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
}

impl Handler<Msg> for Model {
    fn handle(&mut self, msg: &Msg) {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }
}

fn view(model: &Model) -> Html {
    div()
        .push(button().on(click(Msg::Decrement)).push("-"))
        .push(model.count)
        .push(button().on(click(Msg::Increment)).push("+"))
        .into()
}
