mod init;
mod update;
mod view;

pub use self::{init::*, update::*, view::*};

use crate::node::{create_dom_node, patch, ActiveClosures};
use oak_core::{
    future_to_promise,
    futures::{
        sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
        Future, Stream,
    },
    log,
};
use oak_vdom::Node as VirtualNode;
use std::marker::PhantomData;
use wasm_bindgen::JsValue;
use web_sys::Node as BrowserNode;

pub struct App<Model, Msg, Update, View>
where
    Update: Updater<Model, Msg>,
    View: Viewer<Model, Msg>,
{
    model: Model,
    view: View,
    update: Update,
    root: BrowserNode,
    tree: VirtualNode<Msg>,
    closures: ActiveClosures,
}

impl App<(), (), (), ()> {
    pub fn init<Model, Msg, Init>(init: Init) -> WithInit<Model, Msg, Init>
    where
        Init: Initializer<Model, Msg>,
    {
        WithInit {
            phantom: PhantomData,
            init,
        }
    }

    pub fn view<View>(view: View) -> WithView<(), (), (), (), View>
    where
        View: Viewer<(), ()>,
    {
        App::init(()).update(()).view(view)
    }

    pub fn update<Model, Msg, Update>(
        update: Update,
    ) -> WithUpdate<Model, Msg, fn() -> Model, Update>
    where
        Model: Default,
        Update: Updater<Model, Msg>,
    {
        let init: fn() -> Model = Model::default;
        App::init(init).update(update)
    }
}

// pub fn stateless<V>(view: V) -> Stateless
// where
//     V: Into<VirtualNode<()>>,
// {
//     Stateless { view: view.into() }
// }

// pub struct Stateless {
//     view: VirtualNode<()>,
// }

// impl Stateless {
//     pub fn mount(self, selector: &str) -> Result<(), JsValue> {
//         wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
//         let window = web_sys::window().unwrap();
//         let document = window.document().unwrap();
//         let root = document.query_selector(selector)?.unwrap();
//         let node = create_dom_node(&self.view);
//         root.set_inner_html("");
//         root.append_child(&node.node)?;
//         Ok(())
//     }
// }

pub struct WithInit<Model, Msg, Init>
where
    Init: Initializer<Model, Msg>,
{
    phantom: PhantomData<(Model, Msg)>,
    init: Init,
}

impl<Model, Msg, Init> WithInit<Model, Msg, Init>
where
    Init: Initializer<Model, Msg>,
{
    pub fn update<Update>(self, update: Update) -> WithUpdate<Model, Msg, Init, Update>
    where
        Update: Updater<Model, Msg>,
    {
        WithUpdate {
            phantom: PhantomData,
            init: self.init,
            update,
        }
    }
}

// pub fn model<Model>(model: Model) -> WithModel<Model> {
//     WithModel(model)
// }

// pub struct WithModel<Model>(Model);

// impl<Model> WithModel<Model> {
//     pub fn update<Msg>(
//         self,
//         func: impl (Fn(&mut Model, &Msg) -> Box<dyn Cmd<Msg>>) + 'static,
//     ) -> WithUpdate<Model, Msg> {
//         WithUpdate {
//             model_or_init: ModelOrInit::Model(self.0),
//             update: Box::new(func),
//         }
//     }
// }

pub struct WithUpdate<Model, Msg, Init, Update>
where
    Init: Initializer<Model, Msg>,
    Update: Updater<Model, Msg>,
{
    phantom: PhantomData<(Model, Msg)>,
    init: Init,
    update: Update,
}

impl<Model, Msg, Init, Update> WithUpdate<Model, Msg, Init, Update>
where
    Init: Initializer<Model, Msg>,
    Update: Updater<Model, Msg>,
{
    pub fn view<View>(self, view: View) -> WithView<Model, Msg, Init, Update, View>
    where
        View: Viewer<Model, Msg>,
    {
        WithView {
            phantom: PhantomData,
            init: self.init,
            update: self.update,
            view,
        }
    }
}

pub struct WithView<Model, Msg, Init, Update, View>
where
    Init: Initializer<Model, Msg>,
    Update: Updater<Model, Msg>,
    View: Viewer<Model, Msg>,
{
    phantom: PhantomData<(Model, Msg)>,
    init: Init,
    update: Update,
    view: View,
}

pub type AppResult = Result<(), JsValue>;

impl<Model, Msg, Init, Update, View> WithView<Model, Msg, Init, Update, View>
where
    Model: 'static,
    Msg: std::fmt::Debug + 'static,
    Init: Initializer<Model, Msg>,
    Update: Updater<Model, Msg> + 'static,
    View: Viewer<Model, Msg> + 'static,
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
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        let model = self.init.init();
        let tree = self.view.view(&model);
        let (msg_sender, msg_receiver): (UnboundedSender<Msg>, UnboundedReceiver<Msg>) =
            unbounded();
        let root = create_dom_node(&tree, msg_sender.clone());
        let root_node = root.node;
        node.append_child(&root_node)?;

        let update = self.update;
        let view = self.view;

        future_to_promise(
            msg_receiver
                .fold((model, tree), move |(old_model, old_tree), msg| {
                    let new_model = update.update(old_model, msg);
                    let new_tree = view.view(&new_model);
                    let patches = oak_diff::diff(&old_tree, &new_tree);
                    patch(root_node.clone(), &patches, msg_sender.clone()).unwrap();
                    Ok((new_model, new_tree))
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
