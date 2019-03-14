use chrono::Duration;
use oak_core::{
    futures::{
        stream::Stream,
        sync::{mspc, oneshot},
        task::current,
        Async, Poll,
    },
    Sub,
};
use std::{rc::Rc, cell::RefCell};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

pub fn every<Msg>(duration: Duration, msg: Msg) -> EveryFrameSub<Msg> {
    EveryFrameSub {
        window: None,
        msg,
        duration,
        state: InternalState::Init,
    }
}

pub struct EveryFrameSub<Msg> {
    window: Option<web_sys::Window>,
    msg: Msg,
    duration: Duration,
    state: InternalState,
}

enum InternalState {
    Init,
    Running {
        handle: i32,
        closure: Closure<FnMut()>,
        msg_receiver: mspc::Receiver<()>,
        drop_sender: oneshot::Sender<()>,
    },
    Error(JsValue),
    Canceled,
}

impl<Msg> Sub<Msg> for EveryFrameSub<Msg> {}

impl<Msg: Clone> Stream for EveryFrameSub<Msg> {
    type Item = Msg;
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

                let (mut msg_sender, msg_receiver): (Sender<()>, Receiver<()>) = channel(1);
                let task = current();
                let closure = Rc::new(RefCell::new(None));
                let closure_clone = f.clone();
                *closure_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    match drop_receiver.try_recv() {
                        Ok(Some(_)) => return,
                        Err(_) => return,
                    }
                    sender.try_send(()).ok();
                    task.notify();

                }) as Box<FnMut()>));

                let result = window.request_animation_frame(
                    closure.as_ref().unchecked_ref(),
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
                Ok(Async::Ready(_)) => Ok(Async::Ready(Some(self.msg.clone()))),
                _ => Ok(Async::NotReady),
            },
            InternalState::Error(err) => Err(err.clone()),
            InternalState::Canceled => Err(JsValue::from("canceled")),
        }
    }
}

impl<Msg> Drop for EveryFrameSub<Msg> {
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
