use crate::{Cmd, Sub};
use futures::{Async, Future, Poll, Stream};
use wasm_bindgen::prelude::JsValue;

pub struct Idle<Msg>;

impl<Msg> Cmd<Msg> for Idle<Msg> {}

impl<Msg> Future for Idle<Msg> {
    type Item = Msg;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready())
    }
}

impl<Msg> Stream for Idle<Msg> {
    type Item = Msg;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(None))
    }
}
