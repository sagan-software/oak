use crate::browser::BrowserNode;
use crate::events::{Event, EventListeners};
use crate::markup::NodeParent;
use shred::SetupHandler;
use shrev::{EventChannel, ReaderId};
use specs::prelude::{
    Component, DenseVecStorage, Join, Read, ReadStorage, Resources, System, SystemData, VecStorage,
    Write,
};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver};
use wasm_bindgen::{
    prelude::{Closure, JsValue},
    JsCast,
};

pub trait SetupBrowserEvent: Sized {
    fn setup_browser_event(name: &str) -> Receiver<Self>;
}

unsafe impl<T: AsRef<web_sys::Event> + Debug> Send for Event<T> {}

unsafe impl<T: AsRef<web_sys::Event> + Debug> Sync for Event<T> {}

impl<T: AsRef<web_sys::Event> + Debug> SetupBrowserEvent for Event<T> {
    fn setup_browser_event(name: &str) -> Receiver<Self> {
        let (sender, receiver) = channel();
        //        let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        //            web_sys::console::log_2(&JsValue::from("Clicked?"), &e);
        //            match sender.send(MouseEvent(e)) {
        //                Ok(_) => (),
        //                Err(e) => web_sys::console::log_2(&JsValue::from("error:"), &JsValue::from(format!("{:#?}", e))),
        //            }
        //        }) as Box<dyn Fn(_)>);
        //        let et: &web_sys::EventTarget = &web_sys::window().unwrap().document().unwrap();
        //        et
        //            .add_event_listener_with_callback(name, cb.as_ref().unchecked_ref())
        //            .unwrap();
        //        let f = cb.dyn_into::<js_sys::Function>();
        receiver
    }
}

#[derive(Default)]
pub struct BrowserEventSystem<T: AsRef<web_sys::Event> + Debug + 'static, Msg: 'static> {
    pub reader_id: Option<ReaderId<Event<T>>>,
    pub receiver: Option<Receiver<T>>,
    pub phantom: std::marker::PhantomData<(T, Msg)>,
}

impl<'a, T, Msg> System<'a> for BrowserEventSystem<T, Msg>
where
    T: AsRef<web_sys::Event> + Debug + 'static,
    Msg: Send + Sync + 'static,
{
    type SystemData = (
        Read<'a, EventChannel<Event<T>>>,
        Write<'a, EventChannel<Msg>>,
        ReadStorage<'a, EventListeners<T, Msg>>,
        ReadStorage<'a, BrowserNode>,
    );

    fn run(
        &mut self,
        (browser_events, mut msgs, event_listeners, browser_nodes): Self::SystemData,
    ) {
        let mut reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };
        for browser_event in browser_events.read(&mut reader_id) {
            let web_event: &web_sys::Event = browser_event.event.as_ref();
            let event_target = match web_event.target() {
                Some(et) => et,
                None => continue,
            };
            let event_node = match event_target.dyn_ref::<web_sys::Node>() {
                Some(node) => node,
                None => continue,
            };
            for (browser_node, listeners) in (&browser_nodes, &event_listeners).join() {
                if browser_node.node.is_same_node(Some(event_node)) {
                    for listener in &listeners.0 {
                        if listener.name() == browser_event.name {
                            let msg = listener.run(&browser_event.event);
                            msgs.single_write(msg);
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = res
            .try_fetch_mut::<EventChannel<Event<T>>>()
            .map(|mut c| c.register_reader())
            .or_else(|| {
                let mut channel = EventChannel::<Event<T>>::new();
                let reader_id = channel.register_reader();
                res.insert(channel);
                Some(reader_id)
            });
    }
}
