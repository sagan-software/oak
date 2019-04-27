use super::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Children {
    Void,
    Empty,
    Nodes(Vec<Node>),
}

impl From<Vec<Node>> for Children {
    fn from(nodes: Vec<Node>) -> Self {
        Children::Nodes(nodes)
    }
}
