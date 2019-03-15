use std::{any::Any, collections::BTreeMap, rc::Rc};

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
    pub attributes: BTreeMap<String, String>,
    pub events: BTreeMap<String, EventHandler<Msg>>,
    pub children: Children<Msg>,
}

pub struct Attribute(pub String, pub String);

pub struct Event<Msg>(pub String, pub EventHandler<Msg>);

impl<Msg> Element<Msg> {
    pub fn push<T>(mut self, node: T) -> Self
    where
        T: Into<Node<Msg>>,
    {
        match &mut self.children {
            Children::Nodes(ref mut children) => children.push(node.into()),
            Children::Void => self.children = Children::Nodes(vec![node.into()]),
        }
        self
    }

    pub fn push_iter<T, N>(self, nodes: T) -> Self
    where
        T: Iterator<Item = N>,
        N: Into<Node<Msg>>,
    {
        nodes.fold(self, |el, node| el.push(node))
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

    pub fn on(mut self, event: Event<Msg>) -> Self {
        self.events.insert(event.0, event.1);
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Children<Msg> {
    Void,
    Nodes(Vec<Node<Msg>>),
}

#[derive(Clone)]
pub struct EventHandler<Msg>(pub Rc<dyn Fn(web_sys::Event) -> Msg>);

impl<Msg> EventHandler<Msg> {
    pub fn new<F>(func: F) -> Self
    where
        F: 'static + Fn(web_sys::Event) -> Msg,
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
