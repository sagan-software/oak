mod events;
mod from;
mod node;
mod string;
pub use crate::{from::*, node::*, string::*};
use std::collections::BTreeMap;

pub fn element<N: Into<String>, Msg>(name: N) -> Element<Msg> {
    Element {
        namespace: None,
        name: name.into(),
        children: Children::Nodes(Vec::new()),
        attributes: BTreeMap::new(),
        events: BTreeMap::new(),
    }
}

pub fn text<T: Into<String>, Msg>(value: T) -> Node<Msg> {
    Node::Text(value.into())
}

#[macro_export]
macro_rules! declare_elements {
    ($($x:ident)*) => ($(
        pub fn $x<Msg>() -> $crate::Element<Msg> {
            $crate::element(stringify!($x))
        }
    )*)
}

#[macro_export]
macro_rules! declare_text_attributes {
    ($($x:ident, $tag:expr)*) => ($(
        pub fn $x<T: ToString>(value: T) -> $crate::Attribute {
            $crate::Attribute($tag.to_owned(), value.to_string())
        }
    )*);

    ($($x:ident)*) => ($(
        $crate::declare_text_attributes!($x, stringify!($x));
    )*);
}

#[macro_export]
macro_rules! declare_bool_attributes {
    ($($x:ident, $tag:expr)*) => ($(
        pub fn $x() -> $crate::Attribute {
            $crate::Attribute($tag.to_owned(), String::new())
        }
    )*);

    ($($x:ident)*) => ($(
        $crate::declare_bool_attributes!($x, stringify!($x));
    )*);
}
