use crate::markup::{Attribute, Children, Markup};
use std::cell::RefCell;
use std::rc::Rc;

pub use oak_macros::start;
pub use wasm_bindgen::prelude::*;
pub use web_sys::{Node, Text};

pub struct Program<Model, Msg> {
    pub view: Box<Fn(&Model) -> Markup<Msg>>,
    pub update: Box<Fn(&Msg, &mut Model)>,
    pub model: RefCell<Model>,
    pub markup: RefCell<Option<Markup<Msg>>>,
}

pub fn sandbox<Model, Msg, ViewFn, UpdateFn>(
    init: Model,
    view: ViewFn,
    update: UpdateFn,
) -> Program<Model, Msg>
where
    ViewFn: Fn(&Model) -> Markup<Msg> + 'static,
    UpdateFn: Fn(&Msg, &mut Model) + 'static,
{
    Program {
        view: Box::new(view),
        update: Box::new(update),
        model: RefCell::new(init),
        markup: RefCell::new(None),
    }
}

impl<Model, Msg> Program<Model, Msg>
where
    Model: Clone + 'static,
    Msg: PartialEq + Clone + 'static,
{
    pub fn dispatch(&mut self, msg: &Msg) {
        let mut model = self.model.borrow().clone();
        (self.update)(msg, &mut model);
        self.model.replace(model);
        self.render();
    }

    pub fn render(&mut self) {
        let markup = (self.view)(&self.model.borrow());
        self.markup.replace(Some(markup));
    }

    pub fn start(&mut self) {
        self.render()
    }

    // pub fn render_to_element<Msg>(markup: &Markup<Msg>, container: &Node) -> Result<(), JsValue> {
    //     let node: Node = render(markup)?;
    //     container.append_child(&node)?;
    //     Ok(())
    // }

    // pub fn render_to_element_with_id<Msg>(markup: &Markup<Msg>, id: &str) -> Result<(), JsValue> {
    //     let window = web_sys::window().expect("no global `window` exists");
    //     let document = window.document().expect("should have a document on window");
    //     let element = document
    //         .get_element_by_id(id)
    //         .expect("no element found with ID");
    //     render_to_element(markup, &element)
    // }

    // pub fn render_to_body<Msg>(markup: &Markup<Msg>) -> Result<(), JsValue> {
    //     let window = web_sys::window().expect("no global `window` exists");
    //     let document = window.document().expect("should have a document on window");
    //     let body = document.body().expect("no body element found");
    //     render_to_element(markup, &body)
    // }
}

// fn render<Msg>(markup: &Markup<Msg>) -> Result<Node, JsValue> {
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let node: Node = match s {
//         Markup::Tag(tag) => {
//             let el = document.create_element(&tag.name)?;
//             for attribute in tag.attributes.iter() {
//                 match attribute {
//                     Attribute::Text(key, val) => {
//                         el.set_attribute(key, &val)?;
//                     }
//                     Attribute::Bool(key) => {
//                         el.set_attribute(key, "true")?;
//                     }
//                     _ => (),
//                 }
//             }
//             if let Children::Nodes(ref children) = &tag.children {
//                 for child in children.iter() {
//                     render_to_element(child, &el)?;
//                 }
//             }
//             el.into()
//         }
//         Markup::Text(text) => Text::new_with_data(&text)?.into(),
//     };
//     Ok(node)
// }
