use crate::core::Sub;
use chrono::Duration;
use futures::{
    stream::Stream,
    sync::mpsc::{channel, Receiver, Sender},
    task::current,
    Async, Poll,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

pub fn every<Msg>(duration: Duration, msg: Msg) -> EverySub<Msg> {
    EverySub {
        window: None,
        msg,
        duration,
        state: InternalState::Init,
    }
}

pub struct EverySub<Msg> {
    window: Option<web_sys::Window>,
    msg: Msg,
    duration: Duration,
    state: InternalState,
}

impl<Msg: PartialEq> PartialEq for EverySub<Msg> {
    fn eq(&self, other: &Self) -> bool {
        self.msg == other.msg && self.duration == other.duration
    }
}

enum InternalState {
    Init,
    Running(i32, Closure<FnMut()>, Receiver<()>),
    Error(JsValue),
    Canceled,
}

impl<Msg: Clone + PartialEq> Sub<Msg> for EverySub<Msg> {
    fn identity(&self) -> String {
        "".to_string()
    }
}

impl<Msg: Clone> Stream for EverySub<Msg> {
    type Item = Vec<Msg>;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match &mut self.state {
            InternalState::Init => {
                let window = match web_sys::window() {
                    Some(window) => window,
                    None => {
                        let err = JsValue::from("no window");
                        self.state = InternalState::Error(err.clone());
                        return Err(err);
                    }
                };

                let (mut sender, receiver): (Sender<()>, Receiver<()>) = channel(1);
                let task = current();
                let closure = Closure::wrap(Box::new(move || {
                    sender.try_send(()).ok();
                    task.notify();
                }) as Box<FnMut()>);

                let result = window.set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    self.duration.num_milliseconds() as i32,
                );

                self.window = Some(window);

                match result {
                    Ok(handle) => {
                        self.state = InternalState::Running(handle, closure, receiver);
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        self.state = InternalState::Error(err.clone());
                        Err(err)
                    }
                }
            }
            InternalState::Running(_, _, receiver) => match receiver.poll() {
                Ok(Async::Ready(_)) => Ok(Async::Ready(Some(vec![self.msg.clone()]))),
                _ => Ok(Async::NotReady),
            },
            InternalState::Error(err) => Err(err.clone()),
            InternalState::Canceled => Err(JsValue::from("canceled")),
        }
    }
}

impl<Msg> Drop for EverySub<Msg> {
    fn drop(&mut self) {
        if let (Some(window), InternalState::Running(handle, _, receiver)) =
            (&self.window, &mut self.state)
        {
            window.clear_timeout_with_handle(*handle);
            receiver.close();
        }
        self.state = InternalState::Canceled;
    }
}
