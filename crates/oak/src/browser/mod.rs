use hibitset::BitSet;
use specs::prelude::{
    Component, ComponentEvent, Entities, Join, Read, ReaderId, ReadStorage, Resources, System, SystemData, VecStorage, WriteStorage,
};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{console, Document, Event, EventTarget, Node, Window};

use crate::markup::{VirtualNode, VirtualNodeParent};
use crate::specs_hierarchy::Parent;

pub mod events;

pub struct BrowserResources {
    window: Window,
    document: Document,
}

impl Default for BrowserResources {
    fn default() -> Self {
        let window = web_sys::window().expect("window");
        let document = window.document().expect("document");
        Self { window, document }
    }
}

unsafe impl Send for BrowserResources {}

unsafe impl Sync for BrowserResources {}

#[derive(Debug)]
pub struct BrowserNode {
    pub node: Node,
    pub is_mounted: bool,
}

unsafe impl Send for BrowserNode {}

unsafe impl Sync for BrowserNode {}

impl Component for BrowserNode {
    type Storage = VecStorage<Self>;
}

pub struct BrowserNodeCreator;

impl<'a> System<'a> for BrowserNodeCreator {
    type SystemData = (
        Entities<'a>,
        Read<'a, BrowserResources>,
        ReadStorage<'a, VirtualNode>,
        WriteStorage<'a, BrowserNode>,
    );

    fn run(&mut self, (entities, browser, virtual_nodes, mut browser_nodes): Self::SystemData) {
        for (entity, virtual_node) in (&entities, &virtual_nodes).join() {
            if browser_nodes.get(entity).is_none() {
                let node: Node = match virtual_node {
                    VirtualNode::Element(virtual_el) => {
                        let el = browser.document.create_element(&virtual_el.name).unwrap();
                        for (key, val) in &virtual_el.attributes {
                            el.set_attribute(&key, &val).unwrap();
                        }
                        el.into()
                    }
                    VirtualNode::Text(text) => browser.document.create_text_node(&text).into(),
                };
                let browser_node = BrowserNode {
                    node,
                    is_mounted: false,
                };
                browser_nodes.insert(entity, browser_node).unwrap();
            }
        }
    }
}

pub struct BrowserNodeMounter;

impl<'a> System<'a> for BrowserNodeMounter {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, VirtualNodeParent>,
        WriteStorage<'a, BrowserNode>,
    );

    fn run(&mut self, (entities, virtual_node_parents, mut browser_nodes): Self::SystemData) {
        for (entity, virtual_node_parent) in (&entities, &virtual_node_parents).join() {
            let parent_entity = virtual_node_parent.parent_entity();
            let parent_node = match browser_nodes.get(parent_entity) {
                Some(parent) => parent.node.clone(),
                None => continue,
            };
            if let Some(browser_node) = browser_nodes.get_mut(entity) {
                if !browser_node.is_mounted {
                    parent_node.append_child(&(browser_node.node)).unwrap();
                    browser_node.is_mounted = true;
                }
            }
        }
    }
}

#[derive(Default)]
pub struct BrowserNodeUpdater {
    pub dirty: BitSet,
    pub reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for BrowserNodeUpdater {
    type SystemData = (
        ReadStorage<'a, VirtualNode>,
        WriteStorage<'a, BrowserNode>,
    );

    fn run(&mut self, (virtual_nodes, mut browser_nodes): Self::SystemData) {
        web_sys::console::log_1(&JsValue::from(0));
        let reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };

        self.dirty.clear();

        let events = virtual_nodes.channel().read(reader_id);
        for event in events {
            match event {
                ComponentEvent::Modified(id) => {
                    self.dirty.add(*id);
                }
                _ => (),
            }
        }
        web_sys::console::log_1(&JsValue::from(1));
        for (vnode, bnode, _) in (&virtual_nodes, &browser_nodes, &self.dirty).join() {
            web_sys::console::log_1(&JsValue::from(2));
            match vnode {
                VirtualNode::Element(el) => {}
                VirtualNode::Text(text) => {
                    web_sys::console::log_1(&JsValue::from(3));
                    bnode.node.set_text_content(Some(text));
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(WriteStorage::<VirtualNode>::fetch(&res).register_reader());
    }
}
