pub mod attributes;
pub mod events;

mod elements;
pub use self::elements::*;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;

#[derive(Clone, Debug)]
pub enum Html<Msg> {
    Element(Element<Msg>),
    Text(String),
}

impl<T: ToString, Msg> From<T> for Html<Msg> {
    fn from(t: T) -> Html<Msg> {
        Html::Text(t.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Element<Msg> {
    pub name: String,
    pub attrs: Vec<Attribute<Msg>>,
    pub children: Children<Msg>,
}

impl<Msg> From<Element<Msg>> for Html<Msg> {
    fn from(el: Element<Msg>) -> Html<Msg> {
        Html::Element(el)
    }
}

#[derive(Clone, Debug)]
pub enum Children<Msg> {
    SelfClosing,
    Nodes(Vec<Html<Msg>>),
}

impl<Msg> Element<Msg> {
    pub fn key(&self) -> Option<&str> {
        for attr in &self.attrs {
            if let Attribute::Key(ref key) = attr {
                return Some(key);
            }
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute<Msg> {
    Text(String, String),
    Bool(String),
    Key(String),
    Event(EventListener<Msg>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventListener<Msg> {
    pub js_closure: JsClosure,
    pub type_: String,
    pub stop_propagation: bool,
    pub prevent_default: bool,
    pub to_message: EventToMessage<Msg>,
}

#[derive(Clone, Default)]
pub struct JsClosure(pub Rc<RefCell<Option<Closure<Fn(web_sys::Event)>>>>);

impl std::fmt::Debug for JsClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.borrow().is_some() {
            write!(f, "HAS A CLOSURE")
        } else {
            write!(f, "NO CLOSURE")
        }
    }
}

impl std::cmp::PartialEq for JsClosure {
    fn eq(&self, _: &JsClosure) -> bool {
        // This is not good enough to implent Eq, i think
        // And its a bit weird. But it's to ignore this in the Attribute enum
        true
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventToMessage<Msg> {
    StaticMsg(Msg),
}

pub fn text<Msg, S: Into<String>>(inner: S) -> Html<Msg> {
    Html::Text(inner.into())
}
