mod renderer;
mod to_string;

use std::any::Any;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub use self::renderer::Renderer;

#[derive(Clone, Debug)]
pub enum Markup<Msg> {
    Tag(Tag<Msg>),
    Text(String),
    Fragment(Vec<Markup<Msg>>),
    // Comment(String),
    // Doctype(String),
    // Cdata,
    // ProcessingInstruction,
}

#[derive(Clone, Debug)]
pub struct Tag<Msg> {
    pub namespace: Option<String>,
    pub key: Option<String>,
    pub name: String,
    pub attributes: Vec<Attribute<Msg>>,
    pub children: Children<Msg>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute<Msg> {
    Event(EventListener<Msg>),
    Style(String, String),
    Text(String, String),
    Bool(String),
}

#[derive(Clone, Debug)]
pub enum Children<Msg> {
    SelfClosing,
    Nodes(Vec<Markup<Msg>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventListener<Msg> {
    pub js_closure: JsClosure,
    pub name: String,
    pub stop_propagation: bool,
    pub prevent_default: bool,
    pub to_message: EventToMessage<Msg>,
}

#[derive(Clone, Default)]
pub struct JsClosure(pub Rc<RefCell<Option<wasm_bindgen::closure::Closure<Fn(web_sys::Event)>>>>);

pub trait EventClosure<Input, Msg>: Debug {
    fn call_ish(&self, input: Input) -> Msg;
    fn eq_rc(&self, other: &Rc<EventClosure<Input, Msg>>) -> bool;
}

#[derive(Debug)]
pub struct EventClosureImpl<Input, Data, Msg> {
    data: Data,
    func: fn(Data, Input) -> Msg,
}

#[derive(Clone, Debug)]
pub struct RcEventClosure<Input, Msg>(pub Rc<EventClosure<Input, Msg>>);

#[derive(Clone, Debug, PartialEq)]
pub enum EventToMessage<Msg> {
    StaticMsg(Msg),
    Input(fn(String) -> Msg),
    InputWithClosure(RcEventClosure<String, Msg>),
    WithFilter {
        msg: Msg,
        filter: fn(web_sys::Event) -> bool,
    },
}

impl Debug for JsClosure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.borrow().is_some() {
            write!(f, "HAS A CLOSURE")
        } else {
            write!(f, "NO CLOSURE")
        }
    }
}

impl PartialEq for JsClosure {
    fn eq(&self, _: &JsClosure) -> bool {
        // This is not good enough to implent Eq, i think
        // And its a bit weird. But it's to ignore this in the Attribute enum
        true
    }
}

impl<Input, Data, Msg> EventClosureImpl<Input, Data, Msg> {
    pub fn new(data: Data, func: fn(Data, Input) -> Msg) -> Self {
        Self { data, func }
    }
}

impl<Input: Debug + 'static, Data: PartialEq + Debug + Clone + 'static, Msg: Debug + 'static>
    EventClosure<Input, Msg> for EventClosureImpl<Input, Data, Msg>
{
    fn call_ish(&self, input: Input) -> Msg {
        (self.func)(self.data.clone(), input)
    }

    fn eq_rc(&self, other: &Rc<EventClosure<Input, Msg>>) -> bool {
        let other = other as &Any;

        if let Some(other_down) = other.downcast_ref::<EventClosureImpl<Input, Data, Msg>>() {
            self.data == other_down.data && self.func == other_down.func
        } else {
            false
        }
    }
}

impl<Input, Msg> PartialEq for RcEventClosure<Input, Msg> {
    fn eq(&self, other: &RcEventClosure<Input, Msg>) -> bool {
        self.0.eq_rc(&other.0)
    }
}

pub fn text<Msg>(value: &str) -> Markup<Msg> {
    Markup::Text(value.to_owned())
}

pub fn tag<Msg: Clone>(
    name: &str,
    attributes: &[Attribute<Msg>],
    children: &[Markup<Msg>],
) -> Markup<Msg> {
    Markup::Tag(Tag {
        key: None,
        namespace: None,
        name: name.to_owned(),
        attributes: attributes.to_vec(),
        children: Children::Nodes(children.to_vec()),
    })
}

pub fn fragment<Msg: Clone>(nodes: &[Markup<Msg>]) -> Markup<Msg> {
    Markup::Fragment(nodes.to_vec())
}
