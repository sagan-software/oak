use oak::html::{button, div};

#[oak::start]
pub fn main() {
    oak::render((update, view, "body"))
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

fn view(count: i32) -> Html<Msg> {
    div(
        button.onclick(Msg::Decrement)("-"),
        count,
        button.onclick(Msg::Increment)("+"),
    )
}
