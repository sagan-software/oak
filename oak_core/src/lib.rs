use futures::{Async, Future, Poll, Stream};
use wasm_bindgen::prelude::JsValue;

pub trait Task<Msg>: Future<Item = Vec<Msg>, Error = JsValue> + Drop {}

pub trait Sub<Msg>: Stream<Item = Vec<Msg>, Error = JsValue> + Drop {}
