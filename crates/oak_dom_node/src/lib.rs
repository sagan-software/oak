mod from;
pub mod html;
mod to_string;
mod types;
pub use crate::{from::*, to_string::*, types::*};
use std::collections::BTreeMap;

pub fn element<N: Into<String>>(name: N) -> Element {
    Element {
        namespace: None,
        name: name.into(),
        children: Children::Nodes(Vec::new()),
        attributes: BTreeMap::new(),
        events: BTreeMap::new(),
    }
}

pub fn text<T: Into<String>>(value: T) -> Node {
    Node::Text(value.into())
}

#[macro_export]
macro_rules! declare_elements {
    ($($x:ident)*) => ($(
        pub fn $x() -> $crate::Element {
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
