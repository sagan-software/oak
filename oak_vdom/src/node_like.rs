use super::Attributes;

pub enum NodeType {
    Text,
    Element,
}

pub trait NodeLike {
    fn get_type(&self) -> NodeType;
    fn get_attributes(&self) -> &Attributes;
}
