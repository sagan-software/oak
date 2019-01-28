use shrev::EventChannel;
use specs::prelude::{
    Builder, Component, DenseVecStorage, Entity, EntityBuilder, FlaggedStorage, VecStorage, World,
};

use crate::specs_hierarchy::Parent;

#[derive(Clone, Debug)]
pub enum VirtualNode {
    Element(VirtualElement),
    Text(String),
    // Comment(String),
    // Doctype(String),
    // Cdata,
    // ProcessingInstruction,
}

impl Component for VirtualNode {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Clone, Debug)]
pub struct VirtualElement {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: Vec<(String, String)>,
}

pub struct VirtualNodeParent(pub Entity);

impl Component for VirtualNodeParent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Parent for VirtualNodeParent {
    fn parent_entity(&self) -> Entity {
        self.0
    }
}

pub fn text(value: &str) -> VirtualNode {
    VirtualNode::Text(value.to_owned())
}

pub fn create_text<'a>(world: &'a mut World, value: &str) -> EntityBuilder<'a> {
    world.create_entity().with(text(value))
}

pub fn element(name: &str, attributes: &[(&str, &str)]) -> VirtualNode {
    VirtualNode::Element(VirtualElement {
        namespace: None,
        name: name.to_owned(),
        attributes: attributes
            .to_vec()
            .iter()
            .map(|&(k, v)| (k.to_owned(), v.to_owned()))
            .collect(),
    })
}

pub fn create_element<'a>(world: &'a mut World, name: &str, attributes: &[(&str, &str)]) -> EntityBuilder<'a> {
    world.create_entity().with(element(name, attributes))
}

