use std::borrow::Cow;
use std::collections::BTreeMap;
use std::rc::Rc;

pub type CowStr = Cow<'static, str>;

#[derive(Debug, PartialEq, Clone)]
pub enum AttrValue<Msg> {
    Text(CowStr),
    Bool(bool),
    EventHandler(EventHandler<Msg>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node<Msg> {
    Element(Element<Msg>),
    Text(String),
    // Comment(String),
    // Doctype(String),
    // Cdata,
    // ProcessingInstruction,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element<Msg> {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: BTreeMap<String, Option<String>>,
    pub events: BTreeMap<String, EventHandler<Msg>>,
    pub children: Option<Vec<Node<Msg>>>,
}

pub struct Event<Msg>(pub String, pub EventHandler<Msg>);

#[derive(Clone)]
pub struct EventHandler<Msg>(pub Rc<dyn Fn() -> Msg>);

impl<Msg> EventHandler<Msg> {
    pub fn new<F>(func: F) -> Self
    where
        F: 'static + Fn() -> Msg,
    {
        Self(Rc::new(func))
    }
}

impl<Msg> PartialEq for EventHandler<Msg> {
    fn eq(&self, _other: &Self) -> bool {
        // TODO: compare argument/return types
        true
    }
}

impl<Msg> std::fmt::Debug for EventHandler<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "event handler")
    }
}
