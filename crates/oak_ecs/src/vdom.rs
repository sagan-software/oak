use crate::Component;
use std::collections::BTreeMap;

pub enum VirtualNode {
    // Element(Element),
    Text(String),
    // Fragment(Vec<Element>),
}

pub struct VirtualElement {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: BTreeMap<String, String>,
    pub is_self_closing: bool,
}

impl Component for VirtualNode {}
