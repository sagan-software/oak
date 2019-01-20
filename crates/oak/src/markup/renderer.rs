use crate::markup::Markup;

pub trait Renderer<Msg> {
    type Node;
    type Error;
    fn create_node(&self, markup: &Markup<Msg>) -> Result<Self::Node, Self::Error>;
}
