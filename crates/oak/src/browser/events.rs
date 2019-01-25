use std::cell::RefCell;
use std::rc::Rc;

use shred::SetupHandler;
use shrev::{EventChannel, ReaderId};
use specs::prelude::{Component, DenseVecStorage, Join, Read, ReadStorage, Resources, System, SystemData};
use wasm_bindgen::{JsCast, prelude::{Closure, JsValue}};

use crate::browser::BrowserNode;
use crate::markup::VirtualNodeParent;

pub struct MouseEvent(pub web_sys::MouseEvent);

unsafe impl Send for MouseEvent {}

unsafe impl Sync for MouseEvent {}

pub struct MouseEventListener {
    pub name: String,
    pub func: Box<dyn Fn() + Send + Sync>,
}

impl Component for MouseEventListener {
    type Storage = DenseVecStorage<Self>;
}

pub struct MouseEvents {
    pub channel: EventChannel<MouseEvent>
}

unsafe impl Send for MouseEvents {}

unsafe impl Sync for MouseEvents {}

impl Default for MouseEvents {
    fn default() -> Self {
        Self {
            channel: EventChannel::new(),
        }
    }
}

#[derive(Default)]
pub struct MouseEventSystem {
    pub reader_id: Option<ReaderId<MouseEvent>>,
}

impl<'a> System<'a> for MouseEventSystem {
    type SystemData = (
        Read<'a, MouseEvents>,
        ReadStorage<'a, MouseEventListener>,
        ReadStorage<'a, BrowserNode>,
    );

    fn run(&mut self, (mouse_events, mouse_event_listeners, browser_nodes): Self::SystemData) {
        let mut reader_id = match &mut self.reader_id {
            Some(reader_id) => reader_id,
            None => return,
        };
        for mouse_event in mouse_events.channel.read(&mut reader_id) {
            let event_target = match mouse_event.0.target() {
                Some(et) => et,
                None => continue,
            };
            let event_node = match event_target.dyn_ref::<web_sys::Node>() {
                Some(node) => node,
                None => continue,
            };
            for (browser_node, listener) in (&browser_nodes, &mouse_event_listeners).join() {
                if browser_node.node.is_same_node(Some(event_node)) {
                    (listener.func)();
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(res.fetch_mut::<MouseEvents>().channel.register_reader());
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