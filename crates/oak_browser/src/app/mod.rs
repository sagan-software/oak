mod init;
mod subs;
mod update;
mod view;

pub use self::{init::*, subs::*, update::*, view::*};

use crate::node::{create_dom_node, patch, ActiveClosures};
use oak_core::{
    future_to_promise,
    futures::{
        sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
        Future, Stream,
    },
    log, Idle, Sub,
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
    pub fn init<Model, Msg, Init>(init: Init) -> AppBuilder<Model, Msg, Init, (), (), (), Idle<Msg>>
    where
        Init: Initializer<Model, Msg>,
    {
        AppBuilder {
            phantom: PhantomData,
            init,
            update: (),
            view: (),
            subs: (),
        }
    }

    pub fn update<Model, Msg, Update>(
        update: Update,
    ) -> AppBuilder<Model, Msg, Model, Update, (), (), Idle<Msg>>
    where
        Model: Default,
        Update: Updater<Model, Msg>,
    {
        AppBuilder {
            phantom: PhantomData,
            init: Model::default(),
            update,
            view: (),
            subs: (),
        }
    }

    pub fn view<View>(view: View) -> AppBuilder<(), (), (), (), View, (), Idle<()>>
    where
        View: Viewer<(), ()>,
    {
        AppBuilder {
            phantom: PhantomData,
            init: (),
            update: (),
            view,
            subs: (),
        }
    }
}

pub struct AppBuilder<Model, Msg, Init, Update, View, Subs, S>
where
    Init: Initializer<Model, Msg>,
    Update: Updater<Model, Msg>,
    View: Viewer<Model, Msg>,
    Subs: Subscriber<Model, Msg, S>,
    S: Sub<Msg>,
{
    phantom: PhantomData<(Model, Msg, S)>,
    init: Init,
    update: Update,
    view: View,
    subs: Subs,
}

pub type AppResult = Result<(), JsValue>;

impl<Model, Msg, Init, Update, View, Subs, S> AppBuilder<Model, Msg, Init, Update, View, Subs, S>
where
    Init: Initializer<Model, Msg>,
    Update: Updater<Model, Msg>,
    View: Viewer<Model, Msg>,
    Subs: Subscriber<Model, Msg, S>,
    S: Sub<Msg>,
{
    pub fn init<NewInit>(
        self,
        init: NewInit,
    ) -> AppBuilder<Model, Msg, NewInit, Update, View, Subs, S>
    where
        NewInit: Initializer<Model, Msg>,
    {
        AppBuilder {
            phantom: PhantomData,
            init,
            update: self.update,
            view: self.view,
            subs: self.subs,
        }
    }

    pub fn update<NewUpdate>(
        self,
        update: NewUpdate,
    ) -> AppBuilder<Model, Msg, Init, NewUpdate, View, Subs, S>
    where
        NewUpdate: Updater<Model, Msg>,
    {
        AppBuilder {
            phantom: PhantomData,
            init: self.init,
            update,
            view: self.view,
            subs: self.subs,
        }
    }

    pub fn view<NewView>(
        self,
        view: NewView,
    ) -> AppBuilder<Model, Msg, Init, Update, NewView, Subs, S>
    where
        NewView: Viewer<Model, Msg>,
    {
        AppBuilder {
            phantom: PhantomData,
            init: self.init,
            update: self.update,
            view,
            subs: self.subs,
        }
    }

    pub fn subs<NewSubs, NewS>(
        self,
        subs: NewSubs,
    ) -> AppBuilder<Model, Msg, Init, Update, View, NewSubs, NewS>
    where
        NewSubs: Subscriber<Model, Msg, NewS>,
        NewS: Sub<Msg>,
    {
        AppBuilder {
            phantom: PhantomData,
            init: self.init,
            update: self.update,
            view: self.view,
            subs,
        }
    }
}

impl<Model, Msg, Init, Update, View, Subs, S> AppBuilder<Model, Msg, Init, Update, View, Subs, S>
where
    Model: 'static,
    Msg: 'static,
    Init: Initializer<Model, Msg>,
    Update: Updater<Model, Msg> + 'static,
    View: Viewer<Model, Msg> + 'static,
    Subs: Subscriber<Model, Msg, S>,
    S: Sub<Msg> + 'static,
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
        let (msg_sender, msg_receiver): (UnboundedSender<Msg>, UnboundedReceiver<Msg>) =
            unbounded();
        let model = self.init.init(msg_sender.clone());
        let tree = self.view.view(&model);
        let root = create_dom_node(&tree, msg_sender.clone());
        let root_node = root.node;
        node.append_child(&root_node)?;

        let update = self.update;
        let view = self.view;
        let subs = self.subs;

        let s = subs.subs(&model);
        {
            let msg_sender = msg_sender.clone();
            future_to_promise(
                s.for_each(move |msgs| {
                    for msg in msgs.into_iter() {
                        msg_sender.unbounded_send(msg).unwrap();
                    }
                    Ok(())
                })
                .map(|_| JsValue::NULL)
                .map_err(|_| JsValue::NULL),
            );
        };

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
