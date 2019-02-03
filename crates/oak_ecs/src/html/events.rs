use crate::markup::{Attribute, EventClosureImpl, EventListener, EventToMessage, RcEventClosure};
use std::default::Default;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub fn on_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event(EventListener {
        name: "click".to_owned(),
        to_message: EventToMessage::StaticMsg(message),
        stop_propagation: false,
        prevent_default: false,
        js_closure: Default::default(),
    })
}

pub fn on_double_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event(EventListener {
        name: "dblclick".to_owned(),
        to_message: EventToMessage::StaticMsg(message),
        stop_propagation: false,
        prevent_default: false,
        js_closure: Default::default(),
    })
}

pub fn on_blur<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event(EventListener {
        name: "blur".to_owned(),
        to_message: EventToMessage::StaticMsg(message),
        stop_propagation: false,
        prevent_default: false,
        js_closure: Default::default(),
    })
}

// TODO: Ensure that when we start using animationFrame, on_input gets special treatement
pub fn on_input<Msg: 'static>(message: fn(String) -> Msg) -> Attribute<Msg> {
    Attribute::Event(EventListener {
        name: "input".to_owned(),
        to_message: EventToMessage::Input(message),
        stop_propagation: true,
        prevent_default: false,
        js_closure: Default::default(),
    })
}

// TODO: Ensure that when we start using animationFrame, on_input gets special treatement
pub fn on_input2<Msg: 'static + Debug, Data: Debug + Clone + PartialEq + 'static>(
    data: Data,
    message: fn(Data, String) -> Msg,
) -> Attribute<Msg> {
    Attribute::Event(EventListener {
        name: "input".to_owned(),
        to_message: EventToMessage::InputWithClosure(RcEventClosure(Rc::new(
            EventClosureImpl::new(data, message),
        ))),
        stop_propagation: true,
        prevent_default: false,
        js_closure: Default::default(),
    })
}

pub fn on_enter<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event(EventListener {
        name: "keydown".to_owned(),
        to_message: EventToMessage::WithFilter {
            msg: message,
            filter: |event| {
                let key_code = event
                    .dyn_into::<web_sys::KeyboardEvent>()
                    .ok()
                    .map(|ev| ev.key_code())
                    .unwrap_or(0);
                key_code == 13
            },
        },
        prevent_default: false,
        stop_propagation: false,
        js_closure: Default::default(),
    })
}
