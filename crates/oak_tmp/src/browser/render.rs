use std::fmt::Debug;
use std::rc::Rc;

use itertools::{EitherOrBoth, Itertools};
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{self, Document, HtmlElement, Node};

use crate::{
    browser::Program,
    markup::{Attribute, Markup, Tag},
};

pub fn render<Msg: PartialEq + Debug + Clone + 'static, Model: Debug + Clone + 'static>(
    program: &Rc<Program<Model, Msg>>,
    new_tree: &Markup<Msg>,
    old_tree: &Option<Markup<Msg>>,
) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("No Body");
    let parent = document
        .get_element_by_id("app")
        .expect("did not find an app element");

    let parent: Node = parent.into();

    let mut renderer = Renderer {
        document: document,
        program: program.clone(),
        to_remove: vec![],
    };
    // console_log!("New Tree: \n{:#?}\n\nOld Tree: \n{:#?}", new_tree, old_tree);

    // TODO: We should probably not assume that the number here is 0
    renderer.update_element(&parent, Some(new_tree), old_tree.as_ref(), 0)?;
    renderer.remove_to_remove()?;

    // let node = renderer.create_node(new_tree)?;

    // AsRef::<Element>::as_ref(&body).set_inner_html("");
    // AsRef::<Node>::as_ref(&body).append_child(&node)?;

    // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");

    // Right now the class inheritance hierarchy of the DOM isn't super
    // ergonomic, so we manually cast `val: Element` to `&Node` to call the
    // `append_child` method.
    // AsRef::<Element>::as_ref(&body).set_inner_html("");
    // AsRef::<Node>::as_ref(&body).append_child(&node)?;

    Ok(())
}

struct Renderer<Model, Msg> {
    document: Document,
    program: Rc<Program<Model, Msg>>,
    to_remove: Vec<(Node, Node)>,
}

fn eiter_or_both_to_option_tuple<T>(pair: EitherOrBoth<T, T>) -> (Option<T>, Option<T>) {
    use itertools::EitherOrBoth::{Both, Left, Right};
    match pair {
        Both(a, b) => (Some(a), Some(b)),
        Left(a) => (Some(a), None),
        Right(b) => (None, Some(b)),
    }
}

fn parents(node: &Node) -> String {
    let mut result = vec![node.node_name()];
    let mut node = node.to_owned();
    while let Some(new_node) = node.parent_node() {
        result.push(new_node.node_name());
        node = new_node;
    }
    result.join(" -> ")
}

impl<Model, Msg> Renderer<Model, Msg>
where
    Msg: PartialEq + Debug + Clone + 'static,
    Model: Debug + Clone + 'static,
{
    fn update_element(
        &mut self,
        parent: &Node,
        new: Option<&Markup<Msg>>,
        old: Option<&Markup<Msg>>,
        index: u32,
    ) -> Result<(), JsValue> {
        match (old, new) {
            (None, Some(new_html)) => {
                // Node is added
                // console_log!("Adding node");
                parent.append_child(&self.create_node(new_html)?)?;
            }
            (Some(removed), None) => {
                // console_log!("Removing node");
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
                (Markup::Tag(old_tag), Markup::Tag(new_tag))
                    if old_tag.name == new_tag.name && old_tag.key() == new_tag.key() =>
                {
                    // console_log!(
                    //     "Updating {} to {}",
                    //     old_tag.to_html_text(0),
                    //     new_tag.to_html_text(0)
                    // );
                    if let Some(current_node) = parent.child_nodes().item(index) {
                        let current_node: HtmlElement = current_node.dyn_into()?;
                        // We have a node (current_node) that has changed from old_tag to new_tag, though
                        // the tag is still the same. This means we need to diff children and attributes

                        // First we diff attributes
                        // We start by removing the ones that are no longer active
                        for attr in &old_tag.attributes {
                            if !new_tag.attributes.contains(attr) {
                                // console_log!("Removing attribute {:?}", attr);
                                self.remove_attribute(&current_node, attr)?;
                            }

                            // Move closures over to the new events because we do not want them to be garbage collected
                            if attr.is_event() {
                                if let Some(new_attr) =
                                    new_tag.attributes.iter().filter(|e| e == &attr).next()
                                {
                                    if let Some(js_closure) =
                                        attr.get_js_closure().0.borrow_mut().take()
                                    {
                                        new_attr.set_js_closure(js_closure)
                                    }
                                }
                            }
                        }
                        // Then we add the ones that are added
                        for attr in &new_tag.attributes {
                            if !old_tag.attributes.contains(attr) {
                                // console_log!("Adding attribute {:?}", attr);
                                self.add_attribute(&current_node, attr)?;
                            }
                        }

                        for (child_index, pair) in old_tag
                            .children
                            .iter()
                            .zip_longest(new_tag.children.iter())
                            .enumerate()
                        {
                            let (old_child, new_child) = eiter_or_both_to_option_tuple(pair);

                            self.update_element(
                                &current_node.as_ref(),
                                new_child,
                                old_child,
                                child_index as u32,
                            )?;
                        }
                    } else {
                        return Err(JsValue::from_str(&format!(
                            "ERROR: Could not find node at index {}, {:?}",
                            index,
                            parents(parent)
                        )));
                    }
                }
                (Markup::Text(s1), Markup::Text(s2)) => {
                    // Only replace if the text is not the same
                    if s1 != s2 {
                        if let Some(child) = parent.child_nodes().item(index) {
                            parent.replace_child(&self.create_node(new)?, &child)?;
                        } else {
                            return Err(JsValue::from_str(&format!(
                                "ERROR: Could not find node at index {}, {:?}",
                                index,
                                parents(parent)
                            )));
                        }
                    }
                }
                _ => {
                    if let Some(child) = parent.child_nodes().item(index) {
                        parent.replace_child(&self.create_node(new)?, &child)?;
                    } else {
                        return Err(JsValue::from_str(&format!(
                            "ERROR: Could not find node at index {}, {:?}",
                            index,
                            parents(parent)
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

    fn create_node(&self, input: &Markup<Msg>) -> Result<Node, JsValue> {
        // console_log!("Creating node: {:?}", input);

        match input {
            Markup::Tag(Tag {
                name,
                attributes,
                children,
                ..
            }) => {
                let val: HtmlElement = self.document.create_element(&name)?.dyn_into()?;

                for attr in attributes {
                    self.add_attribute(&val, attr)?;
                }

                let val: Node = val.into();

                for child in children {
                    let node = self.create_node(&child)?;
                    val.append_child(&node)?;
                }

                Ok(val)
            }
            Markup::Text(text) => {
                let val = self.document.create_text_node(&text);

                Ok(val.into())
            }
        }
    }

    fn remove_attribute(
        &self,
        node: &HtmlElement,
        attribute: &Attribute<Msg>,
    ) -> Result<(), JsValue> {
        match attribute {
            // Attribute::Key(_) => {}
            // TODO: I think I know why elm normalizes before adding and removing attributes. We should probably do the same
            Attribute::Text(key, _) => {
                Reflect::delete_property(node.as_ref(), &JsValue::from_str(&key))?;
            }
            // Attribute::Style(property, _) => {
            //     node.style().remove_property(property)?;
            // }
            _ => (),
            // Attribute::Event {
            //     type_, js_closure, ..
            // } => {
            //     let closure = js_closure.0.replace(None);

            //     if let Some(closure) = closure {
            //         (node.as_ref() as &web_sys::EventTarget).remove_event_listener_with_callback(
            //             &type_,
            //             closure.as_ref().unchecked_ref(),
            //         )?;
            //     } else {
            //         // console_log!("WARN: Could not get a function to remove listener");
            //     }
            // }
        }

        Ok(())
    }

    fn add_attribute(&self, node: &HtmlElement, attribute: &Attribute<Msg>) -> Result<(), JsValue> {
        match attribute {
            Attribute::Text(key, value) => {
                Reflect::set(
                    node.as_ref(),
                    &JsValue::from_str(&key),
                    &JsValue::from_str(&value),
                )?;
            }

            _ => {} //     Attribute::Event {
                    //         type_,
                    //         to_message,
                    //         stop_propagation,
                    //         prevent_default,
                    //         js_closure,
                    //     } => {
                    //         // console_log!("Adding event {}", type_);
                    //         // let name_for_logging = type_.clone();
                    //         let to_message = to_message.clone();
                    //         let program = self.program.clone();
                    //         let stop_propagation = *stop_propagation;
                    //         let prevent_default = *prevent_default;
                    //         let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
                    //             if prevent_default {
                    //                 event.prevent_default();
                    //             }
                    //             if stop_propagation {
                    //                 event.stop_propagation();
                    //             }
                    //             match &to_message {
                    //                 EventToMessage::StaticMsg(msg) => program.dispatch(msg),
                    //                 EventToMessage::Input(msg_fn) => program.dispatch(&msg_fn(
                    //                     event
                    //                         .target()
                    //                         .and_then(|target| {
                    //                             target.dyn_into::<web_sys::HtmlInputElement>().ok()
                    //                         })
                    //                         .map(|el| el.value())
                    //                         .unwrap_or_default(),
                    //                 )),
                    //                 EventToMessage::InputWithClosure(closure) => program.dispatch(
                    //                     &closure.0.call_ish(
                    //                         event
                    //                             .target()
                    //                             .and_then(|target| {
                    //                                 target.dyn_into::<web_sys::HtmlInputElement>().ok()
                    //                             })
                    //                             .map(|el| el.value())
                    //                             .unwrap_or_default(),
                    //                     ),
                    //                 ),
                    //                 EventToMessage::WithFilter { msg, filter } => {
                    //                     if filter(event) {
                    //                         program.dispatch(msg);
                    //                     }
                    //                 }
                    //             };
                    //             // console_log!("On Event {}!", name_for_logging);
                    //         }) as Box<Fn(_)>);

                    //         (node.as_ref() as &web_sys::EventTarget)
                    //             .add_event_listener_with_callback(&type_, closure.as_ref().unchecked_ref())?;

                    //         // Save the closure somewhere safe so that it will not be freed and invalidated

                    //         let ret = js_closure.0.replace(Some(closure));

                    //         if ret.is_some() {
                    //             console_log!("to_message did already have a closure???");
                    //         }

                    //         // closure.forget();
                    //     }
        }

        Ok(())
    }

    fn remove_to_remove(&self) -> Result<(), JsValue> {
        for (parent, child) in &self.to_remove {
            parent.remove_child(&child)?;
        }
        Ok(())
    }
}
