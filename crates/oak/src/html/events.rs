use crate::html::{Attribute, EventListener, EventToMessage};
use std::default::Default;

pub fn on_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
    Attribute::Event(EventListener {
        type_: "click".to_owned(),
        to_message: EventToMessage::StaticMsg(message),
        stop_propagation: false,
        prevent_default: false,
        js_closure: Default::default(),
    })
}

// pub fn on_double_click<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
//     Attribute::Event(EventListener {
//         type_: "dblclick".to_owned(),
//         to_message: EventToMessage::StaticMsg(message),
//         stop_propagation: false,
//         prevent_default: false,
//         js_closure: Default::default(),
//     })
// }

// pub fn on_blur<Msg: Clone + 'static>(message: Msg) -> Attribute<Msg> {
//     Attribute::Event(EventListener {
//         type_: "blur".to_owned(),
//         to_message: EventToMessage::StaticMsg(message),
//         stop_propagation: false,
//         prevent_default: false,
//         js_closure: Default::default(),
//     })
// }

// // TODO: Ensure that when we start using animationFrame, on_input gets special treatement
// pub fn on_input<Msg: 'static>(message: fn(String) -> Msg) -> Attribute<Msg> {
//     Attribute::Event(EventListener {
//         type_: "input".to_owned(),
//         to_message: EventToMessage::Input(message),
//         stop_propagation: true,
//         prevent_default: false,
//         js_closure: Default::default(),
//     })
// }
