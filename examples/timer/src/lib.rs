use oak::prelude::*;
use oak::time::{After, Every};

const AFTER_SECONDS: i64 = 5;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::init((Model::default(), After::seconds(AFTER_SECONDS, Msg::After)))
        .update(update)
        .view(view)
        .subs(subs)
        .mount_to_body()
}

#[derive(Default)]
struct Model {
    is_after: bool,
    seconds: i32,
    milliseconds: f64,
}

#[derive(Clone, Debug)]
enum Msg {
    After,
    EverySecond,
    EveryFrame(f64),
}

fn update(mut model: Model, msg: Msg) -> Model {
    match msg {
        Msg::After => model.is_after = true,
        Msg::EverySecond => model.seconds += 1,
        Msg::EveryFrame(delta) => model.milliseconds += delta,
    }
    model
}

fn view(model: &Model) -> HtmlElement<Msg> {
    div().push("This page has been open for...").push(
        ul().push(
            li().push(if model.is_after {
                "...at least "
            } else {
                "...less than "
            })
            .push(AFTER_SECONDS)
            .push(" seconds."),
        )
        .push(li().push("...").push(model.seconds).push(" seconds."))
        .push(
            li().push("...")
                .push(model.milliseconds)
                .push(" milliseconds."),
        ),
    )
}

fn subs(model: &Model) -> impl Sub<Msg> {
    Every::seconds(1, Msg::EverySecond)
    // BatchSub::new()
    //     .push(Every::seconds(1, Msg::EverySecond))
    //     .push(Every::seconds(1, Msg::EveryFrame(1000.0)))
}
