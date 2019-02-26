use crate::{
    html::{Attribute, Children, Element, EventListener, EventToMessage, Html},
    program::Program,
};
use itertools::{EitherOrBoth, Itertools};
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Renderer<Model, Msg> {
    program: Rc<Program<Model, Msg>>,
    to_remove: Vec<(web_sys::Node, web_sys::Node)>,
}

fn eiter_or_both_to_option_tuple<T>(pair: EitherOrBoth<T, T>) -> (Option<T>, Option<T>) {
    use itertools::EitherOrBoth::{Both, Left, Right};
    match pair {
        Both(a, b) => (Some(a), Some(b)),
        Left(a) => (Some(a), None),
        Right(b) => (None, Some(b)),
    }
}

impl<Model, Msg> Renderer<Model, Msg>
where
    Msg: PartialEq + Debug + Clone + 'static,
    Model: Debug + Clone + 'static,
{
    pub fn render(
        root: &web_sys::Node,
        program: &Rc<Program<Model, Msg>>,
        new_tree: &Html<Msg>,
        old_tree: &Option<Html<Msg>>,
    ) -> Result<(), JsValue> {
        let mut renderer = Renderer {
            program: program.clone(),
            to_remove: vec![],
        };
        // TODO: We should probably not assume that the number here is 0
        renderer.update_element(root, Some(new_tree), old_tree.as_ref(), 0)?;
        for (parent, child) in &renderer.to_remove {
            parent.remove_child(&child)?;
        }
        Ok(())
    }

    fn update_element(
        &mut self,
        parent: &web_sys::Node,
        new: Option<&Html<Msg>>,
        old: Option<&Html<Msg>>,
        index: u32,
    ) -> Result<(), JsValue> {
        match (old, new) {
            (None, Some(new_html)) => {
                // Node is added
                parent.append_child(&self.create_node(new_html)?)?;
            }
            (Some(_removed), None) => {
                // Node is removed
                if let Some(child) = parent.child_nodes().item(index) {
                    // Don't remove childs until after every iteration is finished. If not, the
                    // indexes will not point to the correct nodes anymore
                    self.to_remove.push((parent.clone(), child));
                } else {
                    // console_log!(
                    //     "Could not find node with index {} when removing {}",
                    //     index,
                    //     removed.to_html_text(0)
                    // );
                }
            }
            (Some(old), Some(new)) => match (old, new) {
                (Html::Element(old_tag), Html::Element(new_tag))
                    if old_tag.name == new_tag.name && old_tag.key() == new_tag.key() =>
                {
                    let current_node: web_sys::Element = match parent.child_nodes().item(index) {
                        Some(n) => n.dyn_into()?,
                        None => {
                            return Err(JsValue::from_str(&format!(
                                "ERROR: Could not find node at index {}",
                                index
                            )));
                        }
                    };
                    // We have a node (current_node) that has changed from old_tag to new_tag, though
                    // the tag is still the same. This means we need to diff children and attributes

                    // First we diff attributes
                    // We start by removing the ones that are no longer active
                    for old_attr in &old_tag.attrs {
                        let new_attr = new_tag.attrs.iter().find(|e| e == &old_attr);
                        if new_attr.is_none() {
                            remove_attribute(&current_node, old_attr)?;
                        } else if let Attribute::Event(old_listener) = old_attr {
                            if let Some(Attribute::Event(new_listener)) = new_attr {
                                if let Some(js_closure) =
                                    old_listener.js_closure.0.borrow_mut().take()
                                {
                                    new_listener.js_closure.0.replace(Some(js_closure));
                                }
                            }
                        }
                    }
                    // Then we add the ones that are added
                    for attr in &new_tag.attrs {
                        if !old_tag.attrs.contains(attr) {
                            self.add_attribute(&current_node, attr)?;
                        }
                    }

                    if let (Children::Nodes(old_children), Children::Nodes(new_children)) =
                        (&old_tag.children, &new_tag.children)
                    {
                        for (child_index, pair) in old_children
                            .iter()
                            .zip_longest(new_children.iter())
                            .enumerate()
                        {
                            let (old_child, new_child) = eiter_or_both_to_option_tuple(pair);

                            self.update_element(
                                &current_node,
                                new_child,
                                old_child,
                                child_index as u32,
                            )?;
                        }
                    }
                }
                (Html::Text(s1), Html::Text(s2)) => {
                    if s1 != s2 {
                        if let Some(child) = parent.child_nodes().item(index) {
                            child.set_text_content(Some(&s2));
                        } else {
                            return Err(JsValue::from_str(&format!(
                                "ERROR: Could not find node at index {}",
                                index,
                            )));
                        }
                    }
                }
                _ => {
                    if let Some(child) = parent.child_nodes().item(index) {
                        parent.replace_child(&self.create_node(new)?, &child)?;
                    } else {
                        return Err(JsValue::from_str(&format!(
                            "ERROR: Could not find node at index {}",
                            index,
                        )));
                    }
                }
            },
            (None, None) => {
                // Should never happen, but if it happens we can just do nothing and it will be okay
            }
        }

        Ok(())
    }

    fn create_node(&self, input: &Html<Msg>) -> Result<web_sys::Node, JsValue> {
        match input {
            Html::Element(Element {
                name,
                attrs,
                children,
                ..
            }) => {
                let el = self.program.browser.document.create_element(&name)?;

                for attr in attrs {
                    self.add_attribute(&el, attr)?;
                }

                let node: web_sys::Node = el.into();

                if let Children::Nodes(children) = children {
                    for child in children {
                        let child_node = self.create_node(&child)?;
                        node.append_child(&child_node)?;
                    }
                }

                Ok(node)
            }
            Html::Text(text) => {
                let node = self.program.browser.document.create_text_node(&text);
                Ok(node.into())
            }
        }
    }

    fn add_attribute(
        &self,
        node: &web_sys::Element,
        attribute: &Attribute<Msg>,
    ) -> Result<(), JsValue> {
        match attribute {
            Attribute::Key(_) => Ok(()),
            Attribute::Text(key, value) => node.set_attribute(&key, &value),
            Attribute::Bool(key) => node.set_attribute(&key, "true"),
            Attribute::Event(EventListener {
                type_,
                to_message,
                stop_propagation,
                prevent_default,
                js_closure,
            }) => {
                let to_message = to_message.clone();
                let program = self.program.clone();
                let stop_propagation = *stop_propagation;
                let prevent_default = *prevent_default;
                let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    if prevent_default {
                        event.prevent_default();
                    }
                    if stop_propagation {
                        event.stop_propagation();
                    }
                    let result = match &to_message {
                        EventToMessage::StaticMsg(msg) => Program::dispatch(&program, msg),
                    };
                    if let Err(error) = result {
                        log::error!("{:#?}", error);
                    }
                }) as Box<Fn(_)>);

                let node_et: &web_sys::EventTarget = &node;
                node_et
                    .add_event_listener_with_callback(&type_, closure.as_ref().unchecked_ref())?;

                let ret = js_closure.0.replace(Some(closure));
                if ret.is_some() {
                    log::warn!("to_message did already have a closure???");
                }
                Ok(())
            }
        }
    }
}

fn remove_attribute<Msg>(
    node: &web_sys::Element,
    attribute: &Attribute<Msg>,
) -> Result<(), JsValue> {
    match attribute {
        Attribute::Key(_) => {}
        // TODO: I think I know why elm normalizes before adding and removing attributes. We should probably do the same
        Attribute::Text(key, _) => {
            node.remove_attribute(key)?;
        }
        Attribute::Bool(key) => {
            node.remove_attribute(key)?;
        }
        Attribute::Event(EventListener {
            type_, js_closure, ..
        }) => {
            if let Some(closure) = js_closure.0.replace(None) {
                let node_et: &web_sys::EventTarget = &node;
                node_et.remove_event_listener_with_callback(
                    &type_,
                    closure.as_ref().unchecked_ref(),
                )?;
            } else {
                log::warn!("Could not get a function to remove listener");
            }
        }
    }

    Ok(())
}
