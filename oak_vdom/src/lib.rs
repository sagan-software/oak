// mod events;
// mod from;
mod declare;
mod element_builder;
mod fmt;
mod node;
mod open_element;
mod void_element;

pub use self::element_builder::ElementBuilder;
pub use self::node::*;
pub use self::open_element::{BareOpenElement, OpenElement, SimpleOpenElement};
pub use self::void_element::{BareVoidElement, VoidElement};

// pub use self::{from::*, node::*, string::*};
// use std::collections::BTreeMap;

// pub fn element<N: Into<String>, Msg>(name: N) -> Element<Msg> {
//     Element {
//         namespace: None,
//         name: name.into(),
//         key: None,
//         children: Children::Nodes(Vec::new()),
//         attributes: BTreeMap::new(),
//         events: BTreeMap::new(),
//     }
// }

// pub fn text<T: Into<String>, Msg>(value: T) -> Node<Msg> {
//     Node::Text(value.into())
// }
