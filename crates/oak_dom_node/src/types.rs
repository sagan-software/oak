use std::{any::Any, collections::BTreeMap, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    // Comment(String),
    // Doctype(String),
    // Cdata,
    // ProcessingInstruction,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: BTreeMap<String, String>,
    pub events: BTreeMap<String, EventHandler>,
    pub children: Children,
}

pub struct Attribute(pub String, pub String);

pub struct Event(pub String, pub EventHandler);

impl Element {
    pub fn push<T>(mut self, node: T) -> Self
    where
        T: Into<Node>,
    {
        match &mut self.children {
            Children::Nodes(ref mut children) => children.push(node.into()),
            Children::Void => self.children = Children::Nodes(vec![node.into()]),
        }
        self
    }

    pub fn void(mut self) -> Self {
        self.children = Children::Void;
        self
    }

    pub fn empty(mut self) -> Self {
        match self.children {
            Children::Nodes(ref mut children) => children.clear(),
            Children::Void => self.children = Children::Nodes(Vec::new()),
        }
        self
    }

    pub fn set(mut self, attr: Attribute) -> Self {
        self.attributes.insert(attr.0, attr.1);
        self
    }

    pub fn on(mut self, event: Event) -> Self {
        self.events.insert(event.0, event.1);
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Children {
    Void,
    Nodes(Vec<Node>),
}

#[derive(Clone)]
pub struct EventHandler(pub Rc<dyn Any>);

impl EventHandler {
    pub fn new<I, O, F>(func: F) -> Self
    where
        I: Any + Sized + 'static,
        O: Any + Sized + 'static,
        F: 'static + Fn(I) -> O,
    {
        Self(Rc::new(func))
    }
}

impl PartialEq for EventHandler {
    fn eq(&self, _other: &Self) -> bool {
        // TODO: compare argument/return types
        true
    }
}

impl std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "event handler")
    }
}
