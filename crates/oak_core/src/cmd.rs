use futures::{Async, Future, Poll};
use wasm_bindgen::prelude::JsValue;

pub trait Cmd<Msg>: Future<Item = Vec<Msg>, Error = JsValue> + Drop {}
