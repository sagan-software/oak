use futures::Stream;
use std::hash::Hash;
use wasm_bindgen::prelude::JsValue;

pub trait Sub<Msg>: Stream<Item = Msg, Error = JsValue> + PartialEq + Drop {}
