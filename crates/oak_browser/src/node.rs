use oak_core::futures::sync::mpsc::UnboundedSender;
use oak_diff::Patch;
use oak_vdom::{Children as VirtualChildren, Element as VirtualElement, Node as VirtualNode};
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, EventTarget, Node, Text};

pub type ActiveClosures = HashMap<u32, Vec<Closure<Fn(web_sys::Event)>>>;

/// A node along with all of the closures that were created for that
/// node's events and all of it's child node's events.
pub struct CreatedNode<T> {
    /// A `Node` or `Element` that was created from a `VirtualNode`
    pub node: T,
    /// A map of a node's unique identifier along with all of the Closures for that node.
    ///
    /// The DomUpdater uses this to look up nodes and see if they're still in the page. If not
    /// the reference that we maintain to their closure will be dropped, thus freeing the Closure's
    /// memory.
    pub closures: ActiveClosures,
}

impl From<CreatedNode<Element>> for CreatedNode<Node> {
    fn from(other: CreatedNode<Element>) -> CreatedNode<Node> {
        CreatedNode {
            node: other.node.into(),
            closures: other.closures,
        }
    }
}

impl<T> CreatedNode<T> {
    pub fn without_closures<N: Into<T>>(node: N) -> Self {
        CreatedNode {
            node: node.into(),
            closures: HashMap::with_capacity(0),
        }
    }
}

/// Apply all of the patches to our old root node in order to create the new root node
/// that we desire.
/// This is usually used after diffing two virtual nodes.
pub fn patch<Msg: 'static, N: Into<Node>>(
    root_node: N,
    patches: &[Patch<Msg>],
    msg_sender: UnboundedSender<Msg>,
) -> Result<ActiveClosures, JsValue> {
    let root_node: Node = root_node.into();

    let mut cur_node_idx = 0;

    let mut nodes_to_find = HashSet::new();

    for patch in patches {
        nodes_to_find.insert(patch.node_idx());
    }

    let mut element_nodes_to_patch = HashMap::new();
    let mut text_nodes_to_patch = HashMap::new();

    // Closures that were added to the DOM during this patch operation.
    let mut active_closures = HashMap::new();

    find_nodes(
        root_node,
        &mut cur_node_idx,
        &mut nodes_to_find,
        &mut element_nodes_to_patch,
        &mut text_nodes_to_patch,
    );

    for patch in patches {
        let patch_node_idx = patch.node_idx();

        if let Some(element) = element_nodes_to_patch.get(&patch_node_idx) {
            let new_closures = apply_element_patch(&element, &patch, msg_sender.clone())?;
            active_closures.extend(new_closures);
            continue;
        }

        if let Some(text_node) = text_nodes_to_patch.get(&patch_node_idx) {
            apply_text_patch(&text_node, &patch, msg_sender.clone())?;
            continue;
        }

        unreachable!("Getting here means we didn't find the element or next node that we were supposed to patch.")
    }

    Ok(active_closures)
}

fn find_nodes(
    root_node: Node,
    cur_node_idx: &mut usize,
    nodes_to_find: &mut HashSet<usize>,
    element_nodes_to_patch: &mut HashMap<usize, Element>,
    text_nodes_to_patch: &mut HashMap<usize, Text>,
) {
    if nodes_to_find.is_empty() {
        return;
    }

    // We use child_nodes() instead of children() because children() ignores text nodes
    let children = root_node.child_nodes();
    let child_node_count = children.length();

    // If the root node matches, mark it for patching
    if nodes_to_find.get(&cur_node_idx).is_some() {
        match root_node.node_type() {
            Node::ELEMENT_NODE => {
                element_nodes_to_patch.insert(*cur_node_idx, root_node.unchecked_into());
            }
            Node::TEXT_NODE => {
                text_nodes_to_patch.insert(*cur_node_idx, root_node.unchecked_into());
            }
            other => unimplemented!("Unsupported root node type: {}", other),
        }
        nodes_to_find.remove(&cur_node_idx);
    }

    *cur_node_idx += 1;

    for i in 0..child_node_count {
        let node = children.item(i).unwrap();

        match node.node_type() {
            Node::ELEMENT_NODE => {
                find_nodes(
                    node,
                    cur_node_idx,
                    nodes_to_find,
                    element_nodes_to_patch,
                    text_nodes_to_patch,
                );
            }
            Node::TEXT_NODE => {
                if nodes_to_find.get(&cur_node_idx).is_some() {
                    text_nodes_to_patch.insert(*cur_node_idx, node.unchecked_into());
                }

                *cur_node_idx += 1;
            }
            Node::COMMENT_NODE => {
                // At this time we do not support user entered comment nodes, so if we see a comment
                // then it was a delimiter created by virtual-dom-rs in order to ensure that two
                // neighboring text nodes did not get merged into one by the browser. So we skip
                // over this virtual-dom-rs generated comment node.
            }
            _other => {
                // Ignoring unsupported child node type
                // TODO: What do we do with this situation? Log a warning?
            }
        }
    }
}

fn apply_element_patch<Msg: 'static>(
    node: &Element,
    patch: &Patch<Msg>,
    msg_sender: UnboundedSender<Msg>,
) -> Result<ActiveClosures, JsValue> {
    let active_closures = HashMap::new();

    match patch {
        Patch::SetAttributes(_node_idx, attributes) => {
            for (attrib_name, attrib_val) in attributes.iter() {
                node.set_attribute(attrib_name, attrib_val)?;
            }

            Ok(active_closures)
        }
        Patch::RemoveAttributes(_node_idx, attributes) => {
            for attrib_name in attributes.iter() {
                node.remove_attribute(attrib_name)?;
            }

            Ok(active_closures)
        }
        Patch::Replace(_node_idx, new_node) => {
            let created_node = create_dom_node(new_node, msg_sender);

            node.replace_with_with_node_1(&created_node.node)?;

            Ok(created_node.closures)
        }
        Patch::TruncateChildren(_node_idx, num_children_remaining) => {
            let children = node.child_nodes();
            let mut child_count = children.length();

            // We skip over any separators that we placed between two text nodes
            //   -> `<!--ptns-->`
            //  and trim all children that come after our new desired `num_children_remaining`
            let mut non_separator_children_found = 0;

            for index in 0 as u32..child_count {
                let child = children
                    .get(min(index, child_count - 1))
                    .expect("Potential child to truncate");

                // If this is a comment node then we know that it is a `<!--ptns-->`
                // text node separator that was created in virtual_node/mod.rs.
                if child.node_type() == Node::COMMENT_NODE {
                    continue;
                }

                non_separator_children_found += 1;

                if non_separator_children_found <= *num_children_remaining as u32 {
                    continue;
                }

                node.remove_child(&child).expect("Truncated children");
                child_count -= 1;
            }

            Ok(active_closures)
        }
        Patch::AppendChildren(_node_idx, new_nodes) => {
            let parent = &node;

            let mut active_closures = HashMap::new();

            for new_node in new_nodes {
                let created_node = create_dom_node(new_node, msg_sender.clone());

                parent.append_child(&created_node.node)?;

                active_closures.extend(created_node.closures);
            }

            Ok(active_closures)
        }
        Patch::ChangeText(_node_idx, _new_node) => {
            unreachable!("Elements should not receive ChangeText patches.")
        }
    }
}

fn apply_text_patch<Msg: 'static>(
    node: &Text,
    patch: &Patch<Msg>,
    msg_sender: UnboundedSender<Msg>,
) -> Result<(), JsValue> {
    match patch {
        Patch::ChangeText(_node_idx, text) => {
            node.set_node_value(Some(&text));
        }
        Patch::Replace(_node_idx, new_node) => {
            node.replace_with_with_node_1(&create_dom_node(new_node, msg_sender).node)?;
        }
        other => {
            unreachable!("Text nodes should only receive ChangeText or Replace patches, not ...")
        }
    };

    Ok(())
}

pub fn create_dom_node<Msg: 'static>(
    node: &VirtualNode<Msg>,
    msg_sender: UnboundedSender<Msg>,
) -> CreatedNode<Node> {
    match node {
        VirtualNode::Text(text_node) => CreatedNode::without_closures(create_text_node(&text_node)),
        VirtualNode::Element(element_node) => create_element_node(&element_node, msg_sender).into(),
    }
}

pub fn create_text_node(text: &str) -> Text {
    let document = web_sys::window().unwrap().document().unwrap();
    document.create_text_node(text)
}

/// Build a DOM element by recursively creating DOM nodes for this element and it's
/// children, it's children's children, etc.
pub fn create_element_node<Msg: 'static>(
    el: &VirtualElement<Msg>,
    msg_sender: UnboundedSender<Msg>,
) -> CreatedNode<Element> {
    let document = web_sys::window().unwrap().document().unwrap();

    let element = document.create_element(&el.name).unwrap();
    let mut closures = HashMap::new();

    el.attributes.iter().for_each(|(name, value)| {
        element
            .set_attribute(name, value)
            .expect("Set element attribute in create element");
    });

    if el.events.len() > 0 {
        // let unique_id = create_unique_identifier();
        let unique_id = 1_u32; // TODO fix

        element
            .set_attribute("data-vdom-id".into(), &unique_id.to_string())
            .expect("Could not set attribute on element");

        let current_elem: &EventTarget = element.dyn_ref().unwrap();

        closures.insert(unique_id, vec![]);

        for (name, handler) in el.events.iter() {
            let to_msg = handler.0.clone();
            let msg_sender = msg_sender.clone();
            let func = Closure::wrap(Box::new(move |e| {
                let msg = (to_msg)(e);
                msg_sender.unbounded_send(msg).unwrap();
            }) as Box<dyn Fn(web_sys::Event)>);
            current_elem
                .add_event_listener_with_callback(name, func.as_ref().unchecked_ref())
                .unwrap();
            func.forget(); // TODO fix
                           // closures.get_mut(&unique_id).unwrap().push(func);
        }
    }

    let mut previous_node_was_text = false;

    if let VirtualChildren::Nodes(children) = &el.children {
        children.iter().for_each(|child| {
            match child {
                VirtualNode::Text(text_node) => {
                    let current_node = element.as_ref() as &web_sys::Node;

                    // We ensure that the text siblings are patched by preventing the browser from merging
                    // neighboring text nodes. Originally inspired by some of React's work from 2016.
                    //  -> https://reactjs.org/blog/2016/04/07/react-v15.html#major-changes
                    //  -> https://github.com/facebook/react/pull/5753
                    //
                    // `ptns` = Percy text node separator
                    if previous_node_was_text {
                        let separator = document.create_comment("");
                        current_node
                            .append_child(separator.as_ref() as &web_sys::Node)
                            .unwrap();
                    }

                    current_node
                        .append_child(&create_text_node(text_node))
                        .unwrap();

                    previous_node_was_text = true;
                }
                VirtualNode::Element(element_node) => {
                    previous_node_was_text = false;

                    let child = create_element_node(&element_node, msg_sender.clone());
                    let child_elem: Element = child.node;

                    closures.extend(child.closures);

                    element.append_child(&child_elem).unwrap();
                }
            }
        });
    }

    CreatedNode {
        node: element,
        closures,
    }
}
