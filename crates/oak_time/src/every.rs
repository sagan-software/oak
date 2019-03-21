use chrono::Duration;
use oak_core::{
    futures::{
        stream::Stream,
        sync::mpsc::{channel, Receiver, Sender},
        task::current,
        Async, Poll,
    },
    Sub,
};
use std::fmt::Debug;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

pub struct Every<Msg> {
    window: Option<web_sys::Window>,
    msg: Vec<Msg>,
    duration: Duration,
    state: InternalState,
}

impl<Msg> Every<Msg> {
    pub fn duration(duration: Duration, msg: Msg) -> Self {
        Self {
            window: None,
            msg: vec![msg],
            duration,
            state: InternalState::Init,
        }
    }

    pub fn minutes(minutes: i64, msg: Msg) -> Self {
        Self::duration(Duration::minutes(minutes), msg)
    }

    pub fn seconds(seconds: i64, msg: Msg) -> Self {
        Self::duration(Duration::seconds(seconds), msg)
    }

    pub fn milliseconds(milliseconds: i64, msg: Msg) -> Self {
        Self::duration(Duration::seconds(milliseconds), msg)
    }
}

enum InternalState {
    Init,
    Running(i32, Closure<FnMut()>, Receiver<()>),
    Error(JsValue),
    Canceled,
}

impl<Msg: Clone + Debug> Sub<Msg> for Every<Msg> {
    fn identity(&self) -> String {
        format!("Every{:#?}{:#?}", self.msg, self.duration)
    }
}

impl<Msg: Clone> Stream for Every<Msg> {
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
                Ok(Async::Ready(_)) => Ok(Async::Ready(Some(self.msg.clone()))),
                _ => Ok(Async::NotReady),
            },
            InternalState::Error(err) => Err(err.clone()),
            InternalState::Canceled => Err(JsValue::from("canceled")),
        }
    }
}

impl<Msg> Drop for Every<Msg> {
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
