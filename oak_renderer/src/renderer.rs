use oak_core::specs::Entity;

pub struct RenderedNode<T> {
    pub virtual_node: oak_vdom::Node,
    pub actual_node: Option<T>,
    pub parent: Option<Entity>,
}
