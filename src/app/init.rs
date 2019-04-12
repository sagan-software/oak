use crate::cmd::Cmd;
use futures::{sync::mpsc::UnboundedSender, Future};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;

pub trait Initializer<Model, Msg> {
    fn init(self, msg_sender: UnboundedSender<Msg>) -> Model;
}

impl<Model, Msg, C> Initializer<Model, Msg> for (Model, C)
where
    Msg: 'static,
    C: Cmd<Msg> + 'static,
{
    fn init(self, msg_sender: UnboundedSender<Msg>) -> Model {
        future_to_promise(
            (self.1)
                .map(move |msgs| {
                    for msg in msgs.into_iter() {
                        msg_sender.unbounded_send(msg).unwrap();
                    }
                    JsValue::NULL
                })
                .map_err(|_| JsValue::NULL),
        );
        self.0
    }
}

impl<Model, Msg> Initializer<Model, Msg> for Model {
    fn init(self, _: UnboundedSender<Msg>) -> Model {
        self
    }
}
