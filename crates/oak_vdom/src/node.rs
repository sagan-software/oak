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
    pub key: Option<String>,
    pub attributes: BTreeMap<String, String>,
    pub events: BTreeMap<String, EventHandler<Msg>>,
    pub children: Children<Msg>,
}

pub enum Attribute<Msg> {
    Text(String, String),
    // Bool(String, bool),
    Event(String, EventHandler<Msg>),
}

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
        // TODO: warn about nodes with missing/duplicate keys?
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

    pub fn set(mut self, attr: Attribute<Msg>) -> Self {
        match attr {
            Attribute::Text(k, v) => {
                self.attributes.insert(k, v);
            }
            Attribute::Event(k, v) => {
                self.events.insert(k, v);
            }
        }
        self
    }

    pub fn set_if(mut self, predicate: bool, attr: Attribute<Msg>) -> Self {
        if predicate {
            self.set(attr)
        } else {
            self
        }
    }

    pub fn set_key(mut self, key: Option<String>) -> Self {
        self.key = key;
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
        false
    }
}

impl<Msg> std::fmt::Debug for EventHandler<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "event handler")
    }
}
