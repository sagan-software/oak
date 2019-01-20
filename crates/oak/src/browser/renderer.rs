use crate::{
    browser::Program,
    markup::{
        Attribute, Children, EventListener, EventToMessage, Markup, Renderer as MarkupRenderer,
    },
};
use itertools::{EitherOrBoth, Itertools};
use js_sys::Reflect;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, EventTarget, HtmlElement, HtmlInputElement, Node};

pub struct Renderer<Model, Msg> {
    document: Document,
    to_remove: Vec<(Node, Node)>,
    program: Rc<Program<Model, Msg>>,
    root: Element,
}

impl<Model, Msg> MarkupRenderer<Msg> for Renderer<Model, Msg>
where
    Msg: PartialEq + Debug + Clone + 'static,
    Model: Debug + Clone + 'static,
{
    type Node = Node;
    type Error = JsValue;
    fn create_node(&self, input: &Markup<Msg>) -> Result<Self::Node, Self::Error> {
        match input {
            Markup::Tag(tag) => {
                let val: HtmlElement = self.document.create_element(&tag.name)?.dyn_into()?;

                for attr in &tag.attributes {
                    self.add_attribute(&val, attr)?;
                }

                let val: Node = val.into();

                if let Children::Nodes(children) = &tag.children {
                    for child in children {
                        let node = self.create_node(&child)?;
                        val.append_child(&node)?;
                    }
                }

                Ok(val)
            }
            Markup::Text(text) => {
                let val = self.document.create_text_node(&text);

                Ok(val.into())
            }
            Markup::Fragment(nodes) => {
                let val: Node = self.document.create_document_fragment().into();

                for node in nodes.iter() {
                    let n = &self.create_node(node)?;
                    val.append_child(&n)?;
                }

                Ok(val)
            }
        }
    }
}

impl<Model, Msg> Renderer<Model, Msg>
where
    Msg: PartialEq + Debug + Clone + 'static,
    Model: Debug + Clone + 'static,
{
    pub fn new(program: &Rc<Program<Model, Msg>>) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let root = document
            .get_element_by_id("app")
            .expect("did not find an app element");

        Renderer {
            document: document,
            to_remove: vec![],
            program: program.clone(),
            root,
        }
    }

    pub fn render(
        &mut self,
        new_markup: &Markup<Msg>,
        old_markup: &Option<Markup<Msg>>,
    ) -> Result<(), JsValue> {
        // TODO: We should probably not assume that the number here is 0
        self.update_element(&self.root.clone(), Some(new_markup), old_markup.as_ref(), 0)?;
        self.remove_to_remove()?;
        Ok(())
    }

    fn update_element(
        &mut self,
        parent: &Node,
        new_markup: Option<&Markup<Msg>>,
        old_markup: Option<&Markup<Msg>>,
        index: usize,
    ) -> Result<(), JsValue> {
        match (old_markup, new_markup) {
            (None, Some(new_html)) => {
                // Node is added
                parent.append_child(&self.create_node(new_html)?)?;
            }
            (Some(_removed), None) => {
                // Node is removed
                if let Some(child) = parent.child_nodes().item(index as u32) {
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
                    if old_tag.name == new_tag.name && old_tag.key == new_tag.key =>
                {
                    if let Some(current_node) = parent.child_nodes().item(index as u32) {
                        let current_node: HtmlElement = current_node.dyn_into()?;
                        // We have a node (current_node) that has changed from old_tag to new_tag, though
                        // the tag is still the same. This means we need to diff children and attributes

                        // First we diff attributes
                        // We start by removing the ones that are no longer active
                        for old_attr in &old_tag.attributes {
                            if !new_tag.attributes.contains(old_attr) {
                                // console_log!("Removing attribute {:?}", attr);
                                self.remove_attribute(&current_node, old_attr)?;
                            }
                            // Move closures over to the new events because we do not want them to be garbage collected
                            else if let Attribute::Event(old_listener) = old_attr {
                                for new_attr in &new_tag.attributes {
                                    if let Attribute::Event(new_listener) = new_attr {
                                        if new_listener == old_listener {
                                            new_listener.js_closure.0.replace(
                                                old_listener.js_closure.0.borrow_mut().take(),
                                            );
                                        }
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

                        match (&old_tag.children, &new_tag.children) {
                            (Children::Nodes(old_children), Children::Nodes(new_children)) => {
                                for (child_index, pair) in old_children
                                    .iter()
                                    .zip_longest(new_children.iter())
                                    .enumerate()
                                {
                                    let (old_child, new_child) =
                                        eiter_or_both_to_option_tuple(pair);

                                    self.update_element(
                                        &current_node.as_ref(),
                                        new_child,
                                        old_child,
                                        child_index,
                                    )?;
                                }
                            }
                            (Children::Nodes(_old_children), Children::SelfClosing) => {
                                // TODO
                            }
                            (Children::SelfClosing, Children::Nodes(_new_children)) => {
                                // TODO
                            }
                            (Children::SelfClosing, Children::SelfClosing) => {
                                // Do nothing
                            }
                        }
                    } else {
                        return Err(JsValue::from_str("ERROR: Could not find node at index"));
                    }
                }
                (Markup::Text(s1), Markup::Text(s2)) => {
                    // Only replace if the text is not the same
                    if s1 != s2 {
                        if let Some(child) = parent.child_nodes().item(index as u32) {
                            parent.replace_child(&self.create_node(new)?, &child)?;
                        } else {
                            return Err(JsValue::from_str("ERROR: Could not find node at index"));
                        }
                    }
                }
                _ => {
                    if let Some(child) = parent.child_nodes().item(index as u32) {
                        parent.replace_child(&self.create_node(new)?, &child)?;
                    } else {
                        return Err(JsValue::from_str("ERROR: Could not find node at index"));
                    }
                }
            },
            (None, None) => {
                // Should never happen, but if it happens we can just do nothing and it will be okay
            }
        }

        Ok(())
    }

    fn remove_attribute(
        &mut self,
        node: &HtmlElement,
        attribute: &Attribute<Msg>,
    ) -> Result<(), JsValue> {
        match attribute {
            // TODO: I think I know why elm normalizes before adding and removing attributes. We should probably do the same
            Attribute::Text(key, _) => {
                Reflect::delete_property(node.as_ref(), &JsValue::from_str(&key))?;
            }
            Attribute::Bool(key) => {
                Reflect::delete_property(node.as_ref(), &JsValue::from_str(&key))?;
            }
            Attribute::Style(property, _) => {
                node.style().remove_property(property)?;
            }
            Attribute::Event(listener) => {
                let closure = listener.js_closure.0.replace(None);

                if let Some(closure) = closure {
                    (node.as_ref() as &EventTarget).remove_event_listener_with_callback(
                        &listener.name,
                        closure.as_ref().unchecked_ref(),
                    )?;
                } else {
                    // console_log!("WARN: Could not get a function to remove listener");
                }
            }
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
            Attribute::Bool(key) => {
                Reflect::set(
                    node.as_ref(),
                    &JsValue::from_str(&key),
                    &JsValue::from_bool(true),
                )?;
            }
            Attribute::Style(property, value) => {
                node.style().set_property(property, value)?;
            }
            Attribute::Event(EventListener {
                name,
                to_message,
                stop_propagation,
                prevent_default,
                js_closure,
            }) => {
                let to_message = to_message.clone();
                let program = self.program.clone();
                let stop_propagation = *stop_propagation;
                let prevent_default = *prevent_default;
                let closure = Closure::wrap(Box::new(move |event: Event| {
                    if prevent_default {
                        event.prevent_default();
                    }
                    if stop_propagation {
                        event.stop_propagation();
                    }
                    let result = match &to_message {
                        EventToMessage::StaticMsg(msg) => Program::dispatch(&program, msg),
                        EventToMessage::Input(msg_fn) => Program::dispatch(
                            &program,
                            &msg_fn(
                                event
                                    .target()
                                    .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                                    .map(|el| el.value())
                                    .unwrap_or_default(),
                            ),
                        ),
                        EventToMessage::InputWithClosure(closure) => Program::dispatch(
                            &program,
                            &closure.0.call_ish(
                                event
                                    .target()
                                    .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                                    .map(|el| el.value())
                                    .unwrap_or_default(),
                            ),
                        ),
                        EventToMessage::WithFilter { msg, filter } => {
                            if filter(event) {
                                Program::dispatch(&program, msg)
                            } else {
                                Ok(())
                            }
                        }
                    };
                }) as Box<Fn(_)>);

                (node.as_ref() as &EventTarget)
                    .add_event_listener_with_callback(&name, closure.as_ref().unchecked_ref())?;

                let ret = js_closure.0.replace(Some(closure));

                if ret.is_some() {
                    // console_log!("to_message did already have a closure???");
                }
            }
        }

        Ok(())
    }

    fn remove_to_remove(&mut self) -> Result<(), JsValue> {
        for (parent, child) in &mut self.to_remove {
            parent.remove_child(&child)?;
        }
        Ok(())
    }
}

fn eiter_or_both_to_option_tuple<T>(pair: EitherOrBoth<T, T>) -> (Option<T>, Option<T>) {
    use itertools::EitherOrBoth::{Both, Left, Right};
    match pair {
        Both(a, b) => (Some(a), Some(b)),
        Left(a) => (Some(a), None),
        Right(b) => (None, Some(b)),
    }
}

#[cfg(test)]
mod tests {
    use crate::html::{attributes::class, div, text, Html};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn simple_text_nodes() {
        assert_eq!(1, 1);
        // let html: Html<()> = text("Test!");
        // let out = oak::browser::render(&html);
        // assert_eq!(out.is_ok(), true);
        // assert_eq!(out.unwrap().text_content(), Some("Test!".to_owned()));
    }

    // #[wasm_bindgen_test]
    // fn simple_tag_nodes() {
    //     let html: Html<()> = div(vec![class("hello world")], vec![text("Foobar")]);
    //     let out = oak::browser::render(&html);
    //     assert_eq!(out.is_ok(), true);
    //     assert_eq!(out.unwrap().text_content(), Some("Foobar".to_owned()));
    // }
}
