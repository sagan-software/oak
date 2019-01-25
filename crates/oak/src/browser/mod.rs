pub mod events;

use specs::prelude::{
    Component, Entities, Join, Read, ReadStorage, System, VecStorage, WriteStorage,
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, Document, Event, EventTarget, Node, Window};
use crate::markup::{VirtualNode, VirtualNodeParent};
use crate::specs_hierarchy::Parent;

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
