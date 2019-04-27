use super::{AttributeValue, Attributes, Children, Text};

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub name: Text,
    pub attrs: Attributes,
    pub children: Children,
}
