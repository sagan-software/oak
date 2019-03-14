use oak::core::futures::stream::*;
use oak::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    app::with_init(init)
        .with_update(update)
        .with_view(view)
        .mount("body")?;
    // let fut = time::after(time::Duration::seconds(1), 0);
    // let promise = future_to_promise(fut.map(|_| {
    //     log::info!("AFTER");
    //     JsValue::from(0)
    // }));
    let fut = time::every(time::Duration::seconds(1), 0).for_each(|_| {
        log::info!("111 EVERY!");
        Ok(())
    });
    let promise = future_to_promise(
        fut.map(|_| JsValue::from(0))
            .map_err(|_| JsValue::from_str("Error")),
    );
    Ok(())
}

struct Model {
    milliseconds: f64,
}

#[derive(Clone)]
enum Msg {
    After,
    Every,
    Frame(f64),
}

fn init() -> (Model, impl Cmd<Msg>) {
    // let f = time::every(time::Duration::seconds(1), 0)
    //     .into_future()
    //     .map(|_| {
    //         log::info!("EVERY");
    //         JsValue::from(0)
    //     })
    //     .map_err(|_| JsValue::from_str("Error"));
    // future_to_promise(f);

    (
        Model { milliseconds: 0.0 },
        time::after(time::Duration::seconds(1), Msg::After),
    )
}

fn update(model: &mut Model, msg: &Msg) -> impl Cmd<Msg> {
    match msg {
        Msg::After => log::info!("After"),
        Msg::Every => log::info!("Every"),
        Msg::Frame(delta) => model.milliseconds += delta,
    }
    time::after(time::Duration::seconds(1), Msg::After)
}

fn view(model: &Model) -> Html<Msg> {
    div()
        .push("This page has been open for ")
        .push(model.milliseconds)
        .push(" milliseconds.")
        .into()
}

fn subs(model: &Model) -> impl Sub<Msg> {
    time::every(time::Duration::seconds(1), Msg::Every)
}
