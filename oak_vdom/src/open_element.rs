use super::{Attribute, Attributes, Children, Element, Node, Text};

#[derive(Debug, PartialEq, Clone)]
pub struct BareOpenElement {
    pub name: &'static str,
}

impl BareOpenElement {
    pub fn empty(self) -> Element {
        Element {
            name: self.name.into(),
            attrs: Attributes::new(),
            children: Children::Empty,
        }
    }
}

impl From<BareOpenElement> for Element {
    fn from(el: BareOpenElement) -> Self {
        el.empty()
    }
}

impl From<BareOpenElement> for Node {
    fn from(el: BareOpenElement) -> Self {
        el.empty().into()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OpenElement {
    pub name: Text,
    pub attrs: Attributes,
}

impl OpenElement {
    pub fn children<C>(self, children: C) -> Element
    where
        C: Into<Children>,
    {
        Element {
            name: self.name,
            attrs: self.attrs,
            children: children.into(),
        }
    }

    pub fn empty(self) -> Element {
        Element {
            name: self.name,
            attrs: self.attrs,
            children: Children::Empty,
        }
    }
}

#[macro_export]
macro_rules! declare_open_elements {
    ($($x:ident)*) => ($(
        #[allow(non_upper_case_globals)]
        pub const $x: $crate::BareOpenElement = $crate::BareOpenElement {
            name: stringify!($x)
        };
    )*)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declare_elements() {
        declare_open_elements! {
            div
            span
        }
        assert_eq!(div, BareOpenElement { name: "div" });
        assert_eq!(span, BareOpenElement { name: "span" });
    }

}
