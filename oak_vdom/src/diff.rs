use crate::vdom::{Children, Node};
use std::{cmp::min, collections::BTreeMap, mem};

#[derive(Debug, PartialEq)]
pub enum Patch<'a, Msg> {
    AppendChildren(NodeIdx, Vec<&'a Node<Msg>>),
    TruncateChildren(NodeIdx, usize),
    Replace(NodeIdx, &'a Node<Msg>),
    SetAttributes(NodeIdx, BTreeMap<&'a str, &'a str>),
    RemoveAttributes(NodeIdx, Vec<&'a str>),
    ChangeText(NodeIdx, &'a str),
}

type NodeIdx = usize;

impl<'a, Msg> Patch<'a, Msg> {
    pub fn node_idx(&self) -> usize {
        match self {
            Patch::AppendChildren(node_idx, _) => *node_idx,
            Patch::TruncateChildren(node_idx, _) => *node_idx,
            Patch::Replace(node_idx, _) => *node_idx,
            Patch::SetAttributes(node_idx, _) => *node_idx,
            Patch::RemoveAttributes(node_idx, _) => *node_idx,
            Patch::ChangeText(node_idx, _) => *node_idx,
        }
    }
}

pub fn diff<'a, Msg>(old: &'a Node<Msg>, new: &'a Node<Msg>) -> Vec<Patch<'a, Msg>> {
    diff_recursive(&old, &new, &mut 0)
}

fn diff_recursive<'a, 'b, Msg>(
    old: &'a Node<Msg>,
    new: &'a Node<Msg>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, Msg>> {
    let mut patches = vec![];
    let mut replace = false;

    // Different enum variants, replace!
    if mem::discriminant(old) != mem::discriminant(new) {
        replace = true;
    }

    if let (Node::Element(old_element), Node::Element(new_element)) = (old, new) {
        // Replace if there are different element tags
        if old_element.name != new_element.name {
            replace = true;
        }

        // Replace if two elements have different keys
        // TODO: More robust key support. This is just an early stopgap to allow you to force replace
        // an element... say if it's event changed. Just change the key name for now.
        // In the future we want keys to be used to create a Patch::ReOrder to re-order siblings
        if old_element.key != new_element.key {
            replace = true;
        }
    }

    // Handle replacing of a node
    if replace {
        patches.push(Patch::Replace(*cur_node_idx, &new));
        if let Node::Element(old_element_node) = old {
            if let Children::Nodes(children) = &old_element_node.children {
                for child in children.iter() {
                    increment_node_idx_for_children(child, cur_node_idx);
                }
            }
        }
        return patches;
    }

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old, new) {
        // We're comparing two text nodes
        (Node::Text(old_text), Node::Text(new_text)) => {
            if old_text != new_text {
                patches.push(Patch::ChangeText(*cur_node_idx, &new_text));
            }
        }

        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            let mut add_attributes: BTreeMap<&str, &str> = BTreeMap::new();
            let mut remove_attributes: Vec<&str> = vec![];

            // TODO: -> split out into func
            for (new_attr_name, new_attr_val) in new_element.attributes.iter() {
                match old_element.attributes.get(new_attr_name) {
                    Some(ref old_attr_val) => {
                        if old_attr_val != &new_attr_val {
                            add_attributes.insert(new_attr_name, new_attr_val);
                        }
                    }
                    None => {
                        add_attributes.insert(new_attr_name, new_attr_val);
                    }
                };
            }

            // TODO: -> split out into func
            for (old_attr_name, old_attr_val) in old_element.attributes.iter() {
                if add_attributes.get(&old_attr_name[..]).is_some() {
                    continue;
                };

                match new_element.attributes.get(old_attr_name) {
                    Some(ref new_attr_val) => {
                        if new_attr_val != &old_attr_val {
                            remove_attributes.push(old_attr_name);
                        }
                    }
                    None => {
                        remove_attributes.push(old_attr_name);
                    }
                };
            }

            if add_attributes.len() > 0 {
                patches.push(Patch::SetAttributes(*cur_node_idx, add_attributes));
            }
            if remove_attributes.len() > 0 {
                patches.push(Patch::RemoveAttributes(*cur_node_idx, remove_attributes));
            }

            if let (Children::Nodes(old_children), Children::Nodes(new_children)) =
                (&old_element.children, &new_element.children)
            {
                let old_child_count = old_children.len();
                let new_child_count = new_children.len();

                if new_child_count > old_child_count {
                    let append_patch: Vec<&'a Node<Msg>> =
                        new_children[old_child_count..].iter().collect();
                    patches.push(Patch::AppendChildren(*cur_node_idx, append_patch))
                }

                if new_child_count < old_child_count {
                    patches.push(Patch::TruncateChildren(*cur_node_idx, new_child_count))
                }

                let min_count = min(old_child_count, new_child_count);
                for index in 0..min_count {
                    *cur_node_idx = *cur_node_idx + 1;
                    let old_child = &old_children[index];
                    let new_child = &new_children[index];
                    patches.append(&mut diff_recursive(&old_child, &new_child, cur_node_idx))
                }
                if new_child_count < old_child_count {
                    for child in old_children[min_count..].iter() {
                        increment_node_idx_for_children(child, cur_node_idx);
                    }
                }
            }
        }
        (Node::Text(_), Node::Element(_)) | (Node::Element(_), Node::Text(_)) => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    //    new_root.create_element()
    patches
}

fn increment_node_idx_for_children<'a, 'b, Msg>(old: &'a Node<Msg>, cur_node_idx: &'b mut usize) {
    *cur_node_idx += 1;
    if let Node::Element(element_node) = old {
        if let Children::Nodes(element_children) = &element_node.children {
            for child in element_children.iter() {
                increment_node_idx_for_children(&child, cur_node_idx);
            }
        }
    }
}
