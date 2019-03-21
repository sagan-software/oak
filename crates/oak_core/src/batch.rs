use crate::{Cmd, Sub};
use futures::{Async, Future, Poll, Stream};
use std::marker::PhantomData;
use wasm_bindgen::prelude::JsValue;

#[derive(Default)]
pub struct BatchCmd<Msg> {
    items: Vec<Box<Cmd<Msg>>>,
}

impl<T> Drop for BatchCmd<T> {
    fn drop(&mut self) {}
}

#[derive(Default)]
pub struct BatchSub<Msg> {
    items: Vec<Box<Sub<Msg>>>,
}

impl<Msg> BatchSub<Msg> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push<T>(mut self, sub: T) -> Self
    where
        T: Sub<Msg> + 'static,
    {
        self.items.push(Box::new(sub));
        self
    }
}

impl<T> Drop for BatchSub<T> {
    fn drop(&mut self) {}
}

impl<Msg> Sub<Msg> for BatchSub<Msg> {
    fn identity(&self) -> String {
        "batch".to_string()
    }
}

impl<Msg> Stream for BatchSub<Msg> {
    type Item = Vec<Msg>;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut items = Vec::new();
        for item in &mut self.items {
            match item.poll() {
                Ok(Async::Ready(option)) => match option {
                    Some(mut new_items) => items.append(&mut new_items),
                    None => (),
                },
                Ok(Async::NotReady) => (),
                Err(err) => return Err(err),
            }
        }
        Ok(Async::Ready(Some(items)))
    }
}
