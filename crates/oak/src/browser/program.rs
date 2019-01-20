use crate::{browser::Renderer, html::Html, platform::sub, Cmd, Sub};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::prelude::JsValue;

pub struct Program<Model, Msg> {
    // pub init: Box<Fn(&Flags) -> ( Model, Box<Cmd<Msg>>),
    view: Box<Fn(&Model) -> Html<Msg>>,
    update: Box<Fn(&Msg, &mut Model) -> Box<Cmd<Msg>>>,
    subscriptions: Box<Fn(&Model) -> Box<Sub<Msg>>>,
    model: RefCell<Model>,
    markup: RefCell<Option<Html<Msg>>>,
}

impl<Model, Msg> Program<Model, Msg>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
{
    pub fn dispatch(program: &Rc<Self>, message: &Msg) -> Result<(), JsValue> {
        let mut model = program.model.borrow().clone();
        let cmd = (program.update)(message, &mut model);
        program.model.replace(model);
        Self::render(program)?;
        cmd.run()
    }

    fn render(program: &Rc<Self>) -> Result<(), JsValue> {
        let tree = (program.view)(&program.model.borrow());
        let mut renderer = Renderer::new(program);
        renderer.render(&tree, &program.markup.borrow())?;
        program.markup.replace(Some(tree));
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
    let program = Program {
        view: Box::new(view),
        update: Box::new(update),
        subscriptions: Box::new(|_| Box::new(sub::None)),
        model: RefCell::new(init),
        markup: RefCell::new(None),
    };
    Program::render(&Rc::new(program))
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
    let (model, cmd) = init();
    let program = Program {
        view: Box::new(view),
        update: Box::new(update),
        subscriptions: Box::new(subscriptions),
        model: RefCell::new(model),
        markup: RefCell::new(None),
    };
    Program::render(&Rc::new(program))
}
