use crate::{html::Html, platform::sub, Cmd, Sub};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, prelude::JsValue, JsCast};
use web_sys::{console, Document, Element, Event, EventTarget, Node};

pub struct Program<Model, Msg> {
    // pub init: Box<Fn(&Flags) -> ( Model, Box<Cmd<Msg>>),
    view: Box<Fn(&Model) -> Html<Msg>>,
    update: Box<Fn(&Msg, &mut Model) -> Box<Cmd<Msg>>>,
    subscriptions: Box<Fn(&Model) -> Box<Sub<Msg>>>,
    model: RefCell<Model>,
    markup: RefCell<Option<Html<Msg>>>,
    events: RefCell<HashMap<String, Closure<Fn(Event)>>>,
    pub document: Document,
    pub to_remove: Vec<(Node, Node)>,
    pub root: Element,
}

impl<Model, Msg> Program<Model, Msg>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
{
    pub fn dispatch(&mut self, message: &Msg) -> Result<(), JsValue> {
        let mut model = self.model.borrow().clone();
        let cmd = (self.update)(message, &mut model);
        self.model.replace(model);
        self.render();
        cmd.run()
    }

    fn render(&mut self) -> Result<(), JsValue> {
        console::log_1(&JsValue::from("RENDER!"));
        let tree = (self.view)(&mut self.model.borrow());
        self.render2(&tree, &mut self.markup.borrow())?;
        self.markup.replace(Some(tree));
        Ok(())
    }
}

pub fn sandbox<Model, Msg, ViewFn, UpdateFn>(
    init: Model,
    view: ViewFn,
    update: UpdateFn,
) -> Result<(), JsValue>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
    ViewFn: Fn(&Model) -> Html<Msg> + 'static,
    UpdateFn: Fn(&Msg, &mut Model) -> Box<Cmd<Msg>> + 'static,
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let root = document
        .get_element_by_id("app")
        .expect("did not find an app element");
    let program = Program {
        view: Box::new(view),
        update: Box::new(update),
        subscriptions: Box::new(|_| Box::new(sub::None)),
        model: RefCell::new(init),
        markup: RefCell::new(None),
        events: RefCell::new(HashMap::new()),
        document: document,
        to_remove: vec![],
        root,
    };
    let mut program_rc = Rc::new(program);
    // let mut program_rc2 = program_rc.clone();
    // let cb = Closure::wrap(Box::new(move |_e: Event| {
    //     console::log_1(&JsValue::from("Balls"));
    //     Program::render(&mut program_rc2).unwrap();
    // }) as Box<dyn Fn(_)>);
    // let document = web_sys::window().unwrap().document().unwrap();
    // let et: &EventTarget = &document;
    // et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
    // program_rc
    //     .events
    //     .borrow_mut()
    //     .insert("click".to_owned(), cb);
    Program::render(&mut program_rc)
}

pub fn element<Model, Msg, InitFn, ViewFn, UpdateFn, SubscriptionsFn>(
    init: InitFn,
    view: ViewFn,
    update: UpdateFn,
    subscriptions: SubscriptionsFn,
) -> Result<(), JsValue>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
    InitFn: Fn() -> (Model, Box<Cmd<Msg>>) + 'static,
    ViewFn: Fn(&Model) -> Html<Msg> + 'static,
    UpdateFn: Fn(&Msg, &mut Model) -> Box<Cmd<Msg>> + 'static,
    SubscriptionsFn: Fn(&Model) -> Box<Sub<Msg>> + 'static,
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let root = document
        .get_element_by_id("app")
        .expect("did not find an app element");
    let (model, cmd) = init();
    let program = Program {
        view: Box::new(view),
        update: Box::new(update),
        subscriptions: Box::new(subscriptions),
        model: RefCell::new(model),
        markup: RefCell::new(None),
        events: RefCell::new(HashMap::new()),
        document: document,
        to_remove: vec![],
        root,
    };
    Program::render(&mut Rc::new(program))
}
