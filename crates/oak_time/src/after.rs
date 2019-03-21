use crate::Duration;
use oak_core::{
    futures::{
        sync::oneshot::{channel, Receiver, Sender},
        task::current,
        Async, Future, Poll,
    },
    Cmd,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

pub struct After<Msg> {
    window: Option<web_sys::Window>,
    msg: Option<Vec<Msg>>,
    duration: Duration,
    state: InternalState,
}

impl<Msg> After<Msg> {
    pub fn duration(duration: Duration, msg: Msg) -> Self {
        Self {
            window: None,
            msg: Some(vec![msg]),
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
    Waiting(i32, Closure<FnMut()>, Option<Receiver<()>>),
    Done,
    Canceled,
    Error(JsValue),
}

impl<Msg> Cmd<Msg> for After<Msg> {}

impl<Msg> Future for After<Msg> {
    type Item = Vec<Msg>;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
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

                let (sender, receiver): (Sender<()>, Receiver<()>) = channel();
                let task = current();
                let closure = {
                    let mut sender = Some(sender);
                    Closure::wrap(Box::new(move || {
                        if let Some(sender) = sender.take() {
                            sender.send(()).ok();
                            task.notify();
                        }
                    }) as Box<FnMut()>)
                };

                let result = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    self.duration.num_milliseconds() as i32,
                );

                self.window = Some(window);

                match result {
                    Ok(handle) => {
                        self.state = InternalState::Waiting(handle, closure, Some(receiver));
                        Ok(Async::NotReady)
                    }
                    Err(err) => {
                        self.state = InternalState::Error(err.clone());
                        Err(err)
                    }
                }
            }
            InternalState::Waiting(_, _, receiver) => match receiver {
                Some(receiver) => match receiver.try_recv() {
                    Ok(option) => match option {
                        Some(_) => {
                            self.state = InternalState::Done;
                            match self.msg.take() {
                                Some(msg) => Ok(Async::Ready(msg)),
                                None => Err(JsValue::from("already done")),
                            }
                        }
                        None => Ok(Async::NotReady),
                    },
                    Err(err) => Err(JsValue::from("already done")),
                },
                None => Err(JsValue::from("already done")),
            },
            InternalState::Done => match self.msg.take() {
                Some(msg) => Ok(Async::Ready(msg)),
                None => Err(JsValue::from("already done")),
            },
            InternalState::Error(err) => Err(err.clone()),
            InternalState::Canceled => Err(JsValue::from("canceled")),
        }
    }
}

impl<Msg> Drop for After<Msg> {
    fn drop(&mut self) {
        if let (Some(window), InternalState::Waiting(handle, _, receiver)) =
            (&self.window, &mut self.state)
        {
            window.clear_timeout_with_handle(*handle);
            if let Some(receiver) = receiver {
                receiver.close();
            }
        }
        self.msg = None;
        self.state = InternalState::Canceled;
    }
}
