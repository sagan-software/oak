use crate::vdom::{Attribute, EventHandler};
use js_sys::{JsString, Reflect};
use wasm_bindgen::JsCast;

pub fn on_click<Msg>(msg: Msg) -> Attribute<Msg>
where
    Msg: Clone + 'static,
{
    let func = move |_| -> Msg { msg.clone() };
    let handler = EventHandler::new(func);
    Attribute::Event("click".to_owned(), handler)
}

pub fn on_double_click<Msg>(msg: Msg) -> Attribute<Msg>
where
    Msg: Clone + 'static,
{
    let func = move |_| -> Msg { msg.clone() };
    let handler = EventHandler::new(func);
    Attribute::Event("doubleclick".to_owned(), handler)
}

pub fn on_submit<Msg>(msg: Msg) -> Attribute<Msg>
where
    Msg: Clone + 'static,
{
    let func = move |_| -> Msg { msg.clone() };
    let handler = EventHandler::new(func);
    Attribute::Event("submit".to_owned(), handler)
}

pub fn on_keydown<Msg, F>(f: F) -> Attribute<Msg>
where
    F: Fn(String) -> Msg + 'static,
{
    let func = move |e: web_sys::Event| -> Msg {
        let string = e
            .dyn_into::<web_sys::KeyboardEvent>()
            .map(|e| e.code())
            .unwrap_or_else(|_| "".to_string());
        (f)(string)
    };
    let handler = EventHandler::new(func);
    Attribute::Event("keydown".to_owned(), handler)
}

pub fn on_input<Msg, F>(f: F) -> Attribute<Msg>
where
    F: Fn(String) -> Msg + 'static,
{
    let func = move |e: web_sys::Event| -> Msg {
        let string = e
            .target()
            .and_then(|t| Reflect::get(&t, &"value".into()).ok())
            .map(|v| JsString::from(v).into())
            .unwrap_or_else(|| "".to_string());
        (f)(string)
    };
    let handler = EventHandler::new(func);
    Attribute::Event("input".to_owned(), handler)
}
