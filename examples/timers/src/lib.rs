use oak::prelude::*;
use oak::time::{Duration, Interval, Timeout};

const AFTER_SECONDS: i64 = 5;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::new()
        .with_init(|dispatch| Model {
            timeout: Timeout::new(Duration::seconds(AFTER_SECONDS), || {
                dispatch(Msg::After)
            }),
            interval: Interval::new(Duration::seconds(1), || {
                dispatch(Msg::Interval)
            }),
            seconds: 0,
            milliseconds: 0.0,
        })
        .with_update(update)
        .with_view(view)
        .mount_to_body()
}

#[derive(Default)]
struct Model {
    timeout: oak::time::Timeout,
    interval: oak::time::Interval,
    seconds: i32,
    milliseconds: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum Msg {
    After,
    EverySecond,
    EveryFrame(f64),
    Reset,
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::After => (),
        Msg::EverySecond => model.seconds += 1,
        Msg::EveryFrame(delta) => model.milliseconds += delta,
        Msg::Reset => {
            model.timeout.restart();
            model.interval.restart();
            model.seconds = 0;
            model.milliseconds = 0.0;
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    div((
        button.on(click(Msg::AddTimer)).push("Add Timer"),
        model.timers.iter().enumerate().map(view_timer),
    ))
}

mod timer {
    struct Timer {}

    #[derive(Clone, Debug, PartialEq)]
    enum Msg {
        Tick(f64),
    }

    fn update(model: &mut Timer, msg: Msg) {}
}

// fn subs(model: &Model) -> impl Sub<Msg> {
//     every(Duration::seconds(1), Msg::EverySecond)
//     // BatchSub::new()
//     //     .push(Every::seconds(1, Msg::EverySecond))
//     //     .push(Every::seconds(1, Msg::EveryFrame(1000.0)))
// }