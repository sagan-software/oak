use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver};

use shred::SetupHandler;
use shrev::{EventChannel, ReaderId};
use specs::prelude::{Component, DenseVecStorage, Join, Read, ReadStorage, Resources, System, SystemData, VecStorage};
use wasm_bindgen::{JsCast, prelude::{Closure, JsValue}};

use crate::browser::BrowserNode;
use crate::markup::VirtualNodeParent;

pub struct MouseEvent(pub web_sys::MouseEvent);

pub trait SetupBrowserEvent: Sized {
    fn setup_browser_event(name: &str) -> Receiver<Self>;
}

unsafe impl Send for MouseEvent {}

unsafe impl Sync for MouseEvent {}

impl SetupBrowserEvent for MouseEvent {
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

pub struct EventListener<Event, Msg> {
    pub name: String,
    pub func: Box<dyn Fn(&Event) -> Msg + Send + Sync>,
}

impl<Event: 'static, Msg: 'static> Component for EventListener<Event, Msg> {
    type Storage = VecStorage<Self>;
}


#[derive(Default)]
pub struct EventSystem<Event: 'static, Msg> {
    pub reader_id: Option<ReaderId<Event>>,
    pub phantom: std::marker::PhantomData<(Event, Msg)>,
    pub receiver: Option<Receiver<Event>>,
}

impl<'a, Event, Msg> System<'a> for EventSystem<Event, Msg>
    where
        Event: SetupBrowserEvent + Send + Sync + 'static,
        Msg: 'static {
    type SystemData = (
        Read<'a, EventChannel<Event>>,
        ReadStorage<'a, EventListener<Event, Msg>>,
        ReadStorage<'a, BrowserNode>,
    );

    fn run(&mut self, (events, event_listeners, browser_nodes): Self::SystemData) {
        let mut reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };
        for event in events.read(&mut reader_id) {
//            let event_target = match event.target() {
//                Some(et) => et,
//                None => continue,
//            };
//            let event_node = match event_target.dyn_ref::<web_sys::Node>() {
//                Some(node) => node,
//                None => continue,
//            };
            for (browser_node, listener) in (&browser_nodes, &event_listeners).join() {
                //if browser_node.node.is_same_node(Some(event_node)) {
                (listener.func)(&event);
                //}
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
        self.receiver = Some(Event::setup_browser_event("click"));
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