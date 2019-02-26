use crate::platform::Sub;
pub use std::time::Duration;
use wasm_bindgen::prelude::JsValue;

#[derive(Default, Debug, Clone)]
pub struct Time(f64);

impl Time {
    pub fn now() -> Self {
        Time(js_sys::Date::now())
    }
}

impl ToString for Time {
    fn to_string(&self) -> String {
        js_sys::Date::new(&JsValue::from(self.0))
            .to_iso_string()
            .into()
    }
}

// pub fn every<Msg>(duration: Duration, Fn() -> Msg) -> Sub<Msg> {
//     let cb2 = Closure::wrap(Box::new(move || {
//         web_sys::console::log_1(&JsValue::from("Balls"));
//     }) as Box<dyn Fn()>);
//     window
//         .set_interval_with_callback_and_timeout_and_arguments_0(cb2.as_ref().unchecked_ref(), 1_000)
//         .unwrap();
//     cb2.forget();
// }
