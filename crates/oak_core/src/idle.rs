use crate::{Cmd, Sub};
use futures::{Async, Future, Poll, Stream};
use std::marker::PhantomData;
use wasm_bindgen::prelude::JsValue;

#[derive(Default)]
pub struct Idle<Msg>(PhantomData<Msg>);

impl<Msg> Idle<Msg> {
    pub fn new() -> Self {
        Idle(PhantomData)
    }
}

impl<Msg> Cmd<Msg> for Idle<Msg> {}

impl<Msg> Future for Idle<Msg> {
    type Item = Vec<Msg>;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(Vec::new()))
    }
}

impl<Msg> Sub<Msg> for Idle<Msg> {
    fn identity(&self) -> String {
        "Idle".to_string()
    }
}

impl<Msg> Stream for Idle<Msg> {
    type Item = Vec<Msg>;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(None))
    }
}

impl<Msg> Drop for Idle<Msg> {
    fn drop(&mut self) {}
}
