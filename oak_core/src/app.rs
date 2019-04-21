use crate::core::{Task, Sub};
use crate::idle::Idle;
use crate::node::{create_dom_node, patch, ActiveClosures};
use crate::vdom::Node as VirtualNode;
use futures::{
    sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    Future, Stream,
};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;
use web_sys::Node as BrowserNode;

pub trait InitFn<Model, Msg> {
    fn init(self, msg_sender: UnboundedSender<Msg>) -> Model;
}

impl<Model, Msg, C> InitFn<Model, Msg> for (Model, C)
where
    Msg: 'static,
    C: Task<Msg> + 'static,
{
    fn init(self, msg_sender: UnboundedSender<Msg>) -> Model {
        future_to_promise(
            (self.1)
                .map(move |msgs| {
                    for msg in msgs.into_iter() {
                        msg_sender.unbounded_send(msg).unwrap();
                    }
                    JsValue::NULL
                })
                .map_err(|_| JsValue::NULL),
        );
        self.0
    }
}

impl<Model, Msg> InitFn<Model, Msg> for Model {
    fn init(self, _: UnboundedSender<Msg>) -> Model {
        self
    }
}

pub trait UpdateFn<Model, Msg> {
    fn update(&self, model: &mut Model, msg: Msg);
}

impl<Model, Msg> UpdateFn<Model, Msg> for () {
    fn update(&self, _: &mut Model, _: Msg) {}
}

impl<Model, Msg, T> UpdateFn<Model, Msg> for T
where
    T: Fn(&mut Model, Msg),
{
    fn update(&self, model: &mut Model, msg: Msg) {
        (self)(model, msg)
    }
}

pub trait ViewFn<Model, Msg> {
    fn view(&self, model: &Model) -> VirtualNode<Msg>;
}

impl<Model, Msg> ViewFn<Model, Msg> for () {
    fn view(&self, _: &Model) -> VirtualNode<Msg> {
        VirtualNode::Text("".to_owned())
    }
}

impl<Model, Msg> ViewFn<Model, Msg> for String {
    fn view(&self, _: &Model) -> VirtualNode<Msg> {
        VirtualNode::Text(self.clone())
    }
}

impl<'a, Model, Msg> ViewFn<Model, Msg> for &'a str {
    fn view(&self, _: &Model) -> VirtualNode<Msg> {
        VirtualNode::Text(self.to_string())
    }
}

// impl<Model, Msg> ViewFn<Model, Msg> for VirtualNode<Msg> {
//     fn view(&self, _: &Model) -> VirtualNode<Msg> {
//         *self.clone()
//     }
// }

// impl<Model, Msg> ViewFn<Model, Msg> for VirtualElement<Msg> {
//     fn view(&self, _: &Model) -> VirtualNode<Msg> {
//         (*self.clone()).into()
//     }
// }

impl<Model, Msg, T, V> ViewFn<Model, Msg> for T
where
    T: Fn(&Model) -> V,
    V: Into<VirtualNode<Msg>>,
{
    fn view(&self, model: &Model) -> VirtualNode<Msg> {
        (self)(model).into()
    }
}

pub trait SubsFn<Model, Msg, S>
where
    S: Sub<Msg>,
{
    fn subs(&self, model: &Model) -> S;
}

impl<Model, Msg> SubsFn<Model, Msg, Idle<Msg>> for () {
    fn subs(&self, _: &Model) -> Idle<Msg> {
        Idle::new()
    }
}

impl<Model, Msg, S, T> SubsFn<Model, Msg, S> for T
where
    T: Fn(&Model) -> S,
    S: Sub<Msg>,
{
    fn subs(&self, model: &Model) -> S {
        (self)(model)
    }
}

pub struct App<Model, Msg, Init, Update, View>
where
    Init: InitFn<Model, Msg>,
    Update: UpdateFn<Model, Msg>,
    View: ViewFn<Model, Msg>,
{
    phantom: PhantomData<(Model, Msg)>,
    init: Init,
    update: Update,
    view: View,
}

impl App<(), (), (), (), ()> {
    pub fn new() -> App<(), (), (), (), ()> {
        Self {
            phantom: PhantomData,
            init: (),
            update: (),
            view: (),
        }
    }
}

pub type AppResult = Result<(), JsValue>;

impl<Model, Msg, Init, Update, View> App<Model, Msg, Init, Update, View>
where
    Init: InitFn<Model, Msg>,
    Update: UpdateFn<Model, Msg>,
    View: ViewFn<Model, Msg>,
{
    pub fn with_init<NewInit>(self, init: NewInit) -> App<Model, Msg, NewInit, Update, View>
    where
        NewInit: InitFn<Model, Msg>,
    {
        App {
            phantom: PhantomData,
            init,
            update: self.update,
            view: self.view,
        }
    }

    pub fn with_update<NewUpdate>(self, update: NewUpdate) -> App<Model, Msg, Init, NewUpdate, View>
    where
        NewUpdate: UpdateFn<Model, Msg>,
    {
        App {
            phantom: PhantomData,
            init: self.init,
            update,
            view: self.view,
        }
    }

    pub fn with_view<NewView>(self, view: NewView) -> App<Model, Msg, Init, Update, NewView>
    where
        NewView: ViewFn<Model, Msg>,
    {
        App {
            phantom: PhantomData,
            init: self.init,
            update: self.update,
            view,
        }
    }
}

#[cfg(feature = "use-log")]
fn setup_log() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
}

#[cfg(not(feature = "use-log"))]
fn setup_log() {}

impl<Model, Msg, Init, Update, View> App<Model, Msg, Init, Update, View>
where
    Model: 'static,
    Msg: 'static,
    Init: InitFn<Model, Msg>,
    Update: UpdateFn<Model, Msg> + 'static,
    View: ViewFn<Model, Msg> + 'static,
{
    pub fn mount(self, selector: &str) -> AppResult {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let node = document.query_selector(selector)?.unwrap();
        self.mount_to_node(node.into())
    }

    pub fn mount_to_body(self) -> AppResult {
        self.mount("body")
    }

    pub fn mount_to_node(self, node: web_sys::Node) -> AppResult {
        setup_log();
        let (msg_sender, msg_receiver): (UnboundedSender<Msg>, UnboundedReceiver<Msg>) =
            unbounded();
        let model = self.init.init(msg_sender.clone());
        let tree = self.view.view(&model);
        let root = create_dom_node(&tree, msg_sender.clone());
        let root_node = root.node;
        node.append_child(&root_node)?;

        let update = self.update;
        let view = self.view;
        // let subs = self.subs;

        // let s = subs.subs(&model);
        // {
        //     let msg_sender = msg_sender.clone();
        //     future_to_promise(
        //         s.for_each(move |msgs| {
        //             for msg in msgs.into_iter() {
        //                 msg_sender.unbounded_send(msg).unwrap();
        //             }
        //             Ok(())
        //         })
        //         .map(|_| JsValue::NULL)
        //         .map_err(|_| JsValue::NULL),
        //     );
        // };

        future_to_promise(
            msg_receiver
                .fold((model, tree), move |(mut model, old_tree), msg| {
                    update.update(&mut model, msg);
                    let new_tree = view.view(&model);
                    let patches = crate::diff::diff(&old_tree, &new_tree);
                    patch(root_node.clone(), &patches, msg_sender.clone()).unwrap();
                    Ok((model, new_tree))
                })
                .map(|_| JsValue::NULL)
                .map_err(|_| JsValue::NULL),
        );
        // let app = App {
        //     model,
        //     update: self.update,
        //     view: self.view,
        //     root: root.into(),
        //     tree,
        //     closures: node.closures,
        // };

        Ok(())
    }
}
