use futures::Future;
use wasm_bindgen::prelude::JsValue;

pub trait Cmd<Msg>: Future<Item = Msg, Error = JsValue> + Drop {}
