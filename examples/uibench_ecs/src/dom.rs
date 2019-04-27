use crate::core::{CowStr, Document};
use hibitset::BitSet;
use specs::prelude::*;
use specs_derive::*;
use specs_hierarchy::{Hierarchy, Parent};
use std::collections::HashMap;
use std::ops::Deref;
use wasm_bindgen::JsCast;

web_sys_wrapper!(HtmlTemplateElement);

impl Default for HtmlTemplateElement {
    fn default() -> Self {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("template")
            .unwrap()
            .unchecked_into::<web_sys::HtmlTemplateElement>()
            .into()
    }
}

web_sys_wrapper!(Element);

pub type CustomElementTemplates = HashMap<CowStr, Element>;

pub fn compile_template<S: Into<CowStr>>(
    world: &mut World,
    template: S,
) -> web_sys::Element {
    // TODO Assumes CompilerTemplate and CustomElementTemplates have been registered
    let template_str = template.into();
    {
        let templates = world.read_resource::<CustomElementTemplates>();
        if let Some(el) = templates.get(&template_str) {
            return el.0.clone_node_with_deep(true).unwrap().unchecked_into();
        }
    }

    let compiler = world.read_resource::<HtmlTemplateElement>();
    compiler.0.set_inner_html(&template_str);
    let template_el = compiler.0.content().first_element_child().unwrap();
    let el = template_el
        .clone_node_with_deep(true)
        .unwrap()
        .unchecked_into::<web_sys::Element>();
    let mut templates = world.write_resource::<CustomElementTemplates>();
    templates.insert(template_str, Element(template_el));
    el
}

// pub type Node = UnsafeWrapper<web_sys::Node>;
web_sys_wrapper!(Node);

impl Component for Node {
    type Storage = VecStorage<Self>;
}

impl From<web_sys::Element> for Node {
    fn from(el: web_sys::Element) -> Self {
        Node(el.into())
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        log::warn!("!!! dropped {:#?}", self);
        if !self.0.is_connected() {
            return;
        }

        if let Some(parent_node) = self.0.parent_node() {
            parent_node.remove_child(&self.0).unwrap();
        }
    }
}

pub struct NodeParent(pub Entity);

impl Component for NodeParent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl specs_hierarchy::Parent for NodeParent {
    fn parent_entity(&self) -> Entity {
        self.0
    }
}

#[derive(Default)]
pub struct NodeSystem {
    pub reader_id: Option<ReaderId<ComponentEvent>>,
    pub inserted: BitSet,
    pub removed: BitSet,
}

impl<'a> System<'a> for NodeSystem {
    type SystemData = (ReadStorage<'a, Node>, ReadStorage<'a, NodeParent>);

    fn run(&mut self, (nodes, parents): Self::SystemData) {
        self.inserted.clear();
        self.removed.clear();

        parents.channel().read(self.reader_id.as_mut().unwrap()).for_each(
            |event| match event {
                ComponentEvent::Inserted(id) => {
                    self.inserted.add(*id);
                }
                ComponentEvent::Removed(id) => {
                    self.removed.add(*id);
                }
                _ => (),
            },
        );

        // TODO remove all children of removed entities in self.removed

        (&nodes, &parents, &self.inserted).join().for_each(
            |(node, parent, _)| {
                if node.0.is_connected() {
                    // TODO verify that node is connected to parent?
                    return;
                }

                let parent_entity = parent.parent_entity();
                let parent_node = match nodes.get(parent_entity) {
                    Some(parent_node) => parent_node,
                    None => return,
                };

                parent_node.0.append_child(&node.0).unwrap();
            },
        );
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id =
            Some(WriteStorage::<NodeParent>::fetch(&res).register_reader());
    }
}
