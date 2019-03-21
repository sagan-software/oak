use futures::Stream;
use wasm_bindgen::prelude::JsValue;

pub trait Sub<Msg>: Stream<Item = Vec<Msg>, Error = JsValue> + Drop {
    fn identity(&self) -> String;
}

