use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver};

use shred::SetupHandler;
use shrev::{EventChannel, ReaderId};
use specs::prelude::{Component, DenseVecStorage, Join, Read, ReadStorage, Resources, System, SystemData, VecStorage, Write};
use wasm_bindgen::{JsCast, prelude::{Closure, JsValue}};

use crate::browser::BrowserNode;
use crate::markup::VirtualNodeParent;

pub struct BrowserEvent<T: AsRef<web_sys::Event>> {
    pub name: String,
    pub event: T,
}

impl<T: AsRef<web_sys::Event>> AsRef<web_sys::Event> for BrowserEvent<T> {
    fn as_ref(&self) -> &web_sys::Event {
        self.event.as_ref()
    }
}

pub trait SetupBrowserEvent: Sized {
    fn setup_browser_event(name: &str) -> Receiver<Self>;
}

unsafe impl<T: AsRef<web_sys::Event>> Send for BrowserEvent<T> {}

unsafe impl<T: AsRef<web_sys::Event>> Sync for BrowserEvent<T> {}

impl<T: AsRef<web_sys::Event>> SetupBrowserEvent for BrowserEvent<T> {
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

pub struct EventListeners<Event, Msg>(pub Vec<(String, Box<dyn Fn(&Event) -> Msg + Send + Sync>)>);

impl<Event: 'static, Msg: 'static> Component for EventListeners<Event, Msg> {
    type Storage = VecStorage<Self>;
}


#[derive(Default)]
pub struct BrowserEventSystem<Event: AsRef<web_sys::Event> + 'static, Msg: 'static> {
    pub reader_id: Option<ReaderId<BrowserEvent<Event>>>,
    pub receiver: Option<Receiver<Event>>,
    pub phantom: std::marker::PhantomData<(Event, Msg)>,
}

impl<'a, Event, Msg> System<'a> for BrowserEventSystem<Event, Msg>
    where
        Event: AsRef<web_sys::Event> + 'static,
        Msg: Send + Sync + 'static {
    type SystemData = (
        Read<'a, EventChannel<BrowserEvent<Event>>>,
        Write<'a, EventChannel<Msg>>,
        ReadStorage<'a, EventListeners<Event, Msg>>,
        ReadStorage<'a, BrowserNode>,
    );

    fn run(&mut self, (browser_events, mut msgs, event_listeners, browser_nodes): Self::SystemData) {
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
                    for (name, func) in &listeners.0 {
                        if name == &browser_event.name {
                            let msg = (func)(&browser_event.event);
                            msgs.single_write(msg);
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = res.try_fetch_mut::<EventChannel<BrowserEvent<Event>>>()
            .map(|mut c| c.register_reader())
            .or_else(|| {
                let mut channel = EventChannel::<BrowserEvent<Event>>::new();
                let reader_id = channel.register_reader();
                res.insert(channel);
                Some(reader_id)
            });
    }
}

pub struct EventLogger<Event: Debug + 'static> {
    pub reader_id: Option<ReaderId<Event>>,
    pub phantom: std::marker::PhantomData<Event>,
}

impl<'a, Event: Send + Sync + Debug + 'static> System<'a> for EventLogger<Event> {
    type SystemData = Read<'a, EventChannel<Event>>;

    fn run(&mut self, msgs: Self::SystemData) {
        let reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };
        for msg in msgs.read(reader_id) {
            web_sys::console::log_2(&JsValue::from("MSG!"), &JsValue::from(format!("{:#?}", msg)));
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = res.try_fetch_mut::<EventChannel<Event>>()
            .map(|mut c| c.register_reader())
            .or_else(|| {
                let mut channel = EventChannel::<Event>::new();
                let reader_id = channel.register_reader();
                res.insert(channel);
                Some(reader_id)
            });
    }
}

//impl<P> SetupHandler<Hierarchy<P>> for HierarchySetupHandler<P>
//    where
//        P: Component + Send + Sync + 'static,
//        P::Storage: Tracked,
//{
//    fn setup(res: &mut Resources) {
//        if !res.has_value::<Hierarchy<P>>() {
//            let hierarchy = {
//                let mut storage: WriteStorage<P> = SystemData::fetch(&res);
//                Hierarchy::<P>::new(storage.register_reader())
//            };
//            res.insert(hierarchy);
//        }
//    }
//}