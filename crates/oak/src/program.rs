use crate::{browser::Resources, html::Html, platform, render, Cmd, Sub};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};

pub struct Program<Model, Msg> {
    pub model: RefCell<Model>,
    pub view: Box<Fn(&Model) -> Html<Msg>>,
    pub update: Box<Fn(&Msg, &mut Model) -> Box<Cmd<Msg>>>,
    pub last_tree: RefCell<Option<Html<Msg>>>,
    pub browser: Resources,
    pub root: web_sys::Node,
}

impl<Model, Msg> Program<Model, Msg>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
{
    fn new<InitFn, ViewFn, UpdateFn>(init: InitFn, view: ViewFn, update: UpdateFn) -> Self
    where
        InitFn: FnOnce() -> (Model, Box<Cmd<Msg>>),
        ViewFn: Fn(&Model) -> Html<Msg> + 'static,
        UpdateFn: Fn(&Msg, &mut Model) -> Box<Cmd<Msg>> + 'static,
    {
        let (model, _) = init();
        let browser = Resources::new().unwrap();
        let root = browser.document.create_document_fragment();
        Self {
            model: RefCell::new(model),
            view: Box::new(view),
            update: Box::new(update),
            last_tree: RefCell::new(None),
            browser,
            root: root.dyn_into().unwrap(),
        }
    }

    pub fn dispatch(program: &Rc<Self>, message: &Msg) -> Result<(), JsValue> {
        let mut model = program.model.borrow().clone();
        let cmd = (program.update)(message, &mut model);
        program.model.replace(model);
        Program::render(program)?;
        cmd.run()
    }

    fn render(program: &Rc<Self>) -> Result<(), JsValue> {
        let tree = (program.view)(&program.model.borrow());
        render::Renderer::render(&program.root, program, &tree, &program.last_tree.borrow())?;
        program.last_tree.replace(Some(tree));
        Ok(())
    }

    pub fn init(mut self, selector: &str) -> Result<(), JsValue> {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        self.root = self
            .browser
            .document
            .query_selector(selector)
            .expect("did not find element")
            .expect("did not find element")
            .dyn_into()?;
        let program = Rc::new(self);
        Program::render(&program)
    }
}

pub fn sandbox<Model, Msg, ViewFn, UpdateFn>(
    init: Model,
    view: ViewFn,
    update: UpdateFn,
) -> Program<Model, Msg>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
    ViewFn: Fn(&Model) -> Html<Msg> + 'static,
    UpdateFn: Fn(&Msg, &mut Model) + 'static,
{
    Program::new(
        move || (init, Box::new(platform::None)),
        view,
        move |msg, model| {
            update(msg, model);
            Box::new(platform::None)
        },
    )
}

pub fn element<
    Model,
    Msg,
    InitCmd,
    InitFn,
    ViewFn,
    UpdateCmd,
    UpdateFn,
    SubscriptionsSub,
    SubscriptionsFn,
>(
    init: InitFn,
    view: ViewFn,
    update: UpdateFn,
    subscriptions: SubscriptionsFn,
) -> Program<Model, Msg>
where
    Model: Debug + Clone + 'static,
    Msg: PartialEq + Debug + Clone + 'static,
    InitCmd: Cmd<Msg> + 'static,
    InitFn: FnOnce() -> (Model, InitCmd),
    ViewFn: Fn(&Model) -> Html<Msg> + 'static,
    UpdateCmd: Cmd<Msg> + 'static,
    UpdateFn: Fn(&Msg, &mut Model) -> UpdateCmd + 'static,
    SubscriptionsSub: Sub<Msg> + 'static,
    SubscriptionsFn: Fn(&Model) -> SubscriptionsSub,
{
    Program::new(
        move || {
            let (model, cmd) = init();
            (model, Box::new(cmd))
        },
        view,
        move |msg, model| Box::new(update(msg, model)),
    )
}
