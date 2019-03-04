#[cfg(feature = "browser")]
mod browser;
#[cfg(feature = "browser")]
pub use crate::browser::*;

use oak_core::{
    BitSet, Builder, Component, DenseVecStorage, Entity, EntityBuilder, EventChannel,
    FlaggedStorage, Hierarchy, HierarchySetupHandler, Join, Parent, Read, ReadStorage, System,
    VecStorage, World, Write,
};
use std::collections::{BTreeSet, BTreeMap};

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
    pub attributes: BTreeMap<String, String>,
    pub is_self_closing: bool,
}

impl VirtualElement {
    pub fn new(name: &str) -> Self {
        Self {
            namespace: None,
            name: name.to_owned(),
            attributes: BTreeMap::new(),
            is_self_closing: false,
        }
    }

    pub fn into_node(self) -> VirtualNode {
        VirtualNode::Element(self)
    }
}

pub struct ParentNode(pub Entity);

impl Component for ParentNode {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Parent for ParentNode {
    fn parent_entity(&self) -> Entity {
        self.0
    }
}

#[derive(Default)]
pub struct StringConfig {
    pub roots: BTreeSet<Entity>,
}

#[derive(Default)]
pub struct StringRenderer;

impl StringRenderer {
    fn push_entities<'a, T: Iterator<Item = &'a Entity>>(
        out: &mut String,
        root_entities: T,
        nodes: &ReadStorage<'a, VirtualNode>,
        hierarchy: &Read<'a, Hierarchy<ParentNode>, HierarchySetupHandler<ParentNode>>,
    ) {
        for root_entity in root_entities {
            if let Some(root_node) = nodes.get(*root_entity) {
                Self::push_entity(out, root_entity, root_node, &nodes, &hierarchy);
            }
        }
    }

    fn push_entity<'a>(
        out: &mut String,
        root_entity: &Entity,
        root_node: &VirtualNode,
        nodes: &ReadStorage<'a, VirtualNode>,
        hierarchy: &Read<'a, Hierarchy<ParentNode>, HierarchySetupHandler<ParentNode>>,
    ) {
        match root_node {
            VirtualNode::Element(el) => {
                Self::push_open_tag(out, &el);
                if !el.is_self_closing {
                    let children = hierarchy.children(*root_entity);
                    Self::push_entities(out, children.iter(), nodes, hierarchy);
                    Self::push_close_tag(out, &el);
                }
            }
            VirtualNode::Text(text) => {
                out.push_str(&text);
            }
        }
    }

    fn push_open_tag(out: &mut String, el: &VirtualElement) {
        out.push('<');
        Self::push_ns(out, &el.namespace);
        out.push_str(&el.name);
        Self::push_attrs(out, &el.attributes);
        if el.is_self_closing {
            out.push_str("/>");
        } else {
            out.push('>');
        }
    }

    fn push_ns(out: &mut String, namespace: &Option<String>) {
        if let Some(ns) = namespace {
            out.push_str(&ns);
            out.push(':');
        }
    }

    fn push_attrs(out: &mut String, attrs: &BTreeMap<String, String>) {
        for attr in attrs {
            out.push(' ');
            for (k, v) in attrs.iter() {
                out.push_str(k);
                // TODO: escape
                out.push_str("='");
                out.push_str(v);
                out.push('\'');
            }
        }
    }

    fn push_close_tag(out: &mut String, el: &VirtualElement) {
        out.push_str("</");
        Self::push_ns(out, &el.namespace);
        out.push_str(&el.name);
        out.push('>');
    }
}

impl<'a> System<'a> for StringRenderer {
    type SystemData = (
        ReadStorage<'a, VirtualNode>,
        Read<'a, Hierarchy<ParentNode>, HierarchySetupHandler<ParentNode>>,
        Read<'a, StringConfig>,
        Write<'a, String>,
    );

    fn run(&mut self, (nodes, hierarchy, config, mut out): Self::SystemData) {
        out.clear();
        Self::push_entities(&mut out, config.roots.iter(), &nodes, &hierarchy);
    }
}

#[cfg(test)]
mod tests {
    use super::{Attribute, Element, Node, NodeParent, StringConfig, StringRenderer};
    use crate::specs_hierarchy::HierarchySystem;
    use specs::prelude::{Builder, Dispatcher, DispatcherBuilder, EntityBuilder, World};
    use std::ops::Deref;

    #[test]
    fn basic() {
        let mut world = World::new();
        let mut dispatcher = DispatcherBuilder::new()
            .with(HierarchySystem::<NodeParent>::new(), "node_tree", &[])
            .with(StringRenderer, "string_renderer", &[])
            .build();
        dispatcher.setup(&mut world.res);

        let root = world
            .create_entity()
            .with(Node::Element(Element {
                namespace: None,
                name: "div".to_owned(),
                attributes: vec![],
                is_self_closing: false,
            }))
            .build();
        {
            let mut config = world.write_resource::<StringConfig>();
            config.roots.insert(root);
        }
        dispatcher.dispatch(&world.res);
        world.maintain();
        assert_eq!(world.read_resource::<String>().deref(), "<div></div>");

        world
            .create_entity()
            .with(Node::Text("Hello World".to_owned()))
            .with(NodeParent(root))
            .build();

        dispatcher.dispatch(&world.res);
        world.maintain();
        assert_eq!(
            world.read_resource::<String>().deref(),
            "<div>Hello World</div>"
        );

        let txt = world
            .create_entity()
            .with(Node::Text(" and Foo Bar".to_owned()))
            .with(NodeParent(root))
            .build();

        dispatcher.dispatch(&world.res);
        world.maintain();
        assert_eq!(
            world.read_resource::<String>().deref(),
            "<div>Hello World and Foo Bar</div>"
        );

        {
            let mut config = world.write_resource::<StringConfig>();
            config.roots.insert(txt);
        }

        dispatcher.dispatch(&world.res);
        world.maintain();
        assert_eq!(
            world.read_resource::<String>().deref(),
            "<div>Hello World and Foo Bar</div> and Foo Bar"
        );

        let a = world
            .create_entity()
            .with(Node::Element(Element {
                namespace: None,
                name: "a".to_owned(),
                attributes: vec![Attribute::Text("href".to_owned(), "#".to_owned())],
                is_self_closing: false,
            }))
            .with(NodeParent(root))
            .build();
        world
            .create_entity()
            .with(Node::Text("Test Link".to_owned()))
            .with(NodeParent(a))
            .build();

        dispatcher.dispatch(&world.res);
        world.maintain();
        assert_eq!(
            world.read_resource::<String>().deref(),
            "<div>Hello World and Foo Bar<a href='#'>Test Link</a></div> and Foo Bar"
        );
    }

}
