use crate::core::{CowStr, Document, Event};
use crate::dom::Node;
use shrev::{EventChannel, ReaderId};
use specs::prelude::*;
use specs_derive::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

#[derive(Component)]
#[storage(VecStorage)]
pub struct EventListener {
    pub name: CowStr,
    pub func: Box<dyn Fn(&web_sys::Event) + Send + Sync>,
}

#[derive(Default)]
pub struct EventDelgationSystem {
    reader_id: Option<ReaderId<(CowStr, Event)>>,
    handlers: HashMap<CowStr, ()>, // TODO store closure
}

impl<'a> System<'a> for EventDelgationSystem {
    type SystemData = (
        ReadStorage<'a, EventListener>,
        ReadStorage<'a, Node>,
        Read<'a, Document>,
        Write<'a, EventChannel<(CowStr, Event)>>,
    );

    fn run(&mut self, (listeners, nodes, document, mut channel): Self::SystemData) {
        {
            // Setup new event listeners
            let handlers = &mut self.handlers;
            listeners.join().for_each(move |e| {
                if handlers.contains_key(&e.name) {
                    return;
                }
                log::info!("!!!!!!!!!! SETTING UP EVENT {:#?}", e.name);
                let callback = {
                    let name = e.name.clone();
                    Closure::wrap(Box::new(move |event: web_sys::Event| {
                        log::info!("SENDING EVENT {:#?}", event);
                        // channel.single_write((name, Event(event)));
                    }) as Box<FnMut(web_sys::Event)>)
                };
                document
                    .add_event_listener_with_callback(&e.name, callback.as_ref().unchecked_ref())
                    .unwrap();
                callback.forget(); // TODO store closure
                handlers.insert(e.name.clone(), ());
            });
        }

        {
            // Dispatch events
            channel
                .read(&mut self.reader_id.as_mut().unwrap())
                .for_each(|(name, event)| {
                    log::info!("RECEIVED EVENT ON CHANNEL {:#?}", name);
                });
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = res
            .try_fetch_mut::<EventChannel<(CowStr, Event)>>()
            .map(|mut c| c.register_reader())
            .or_else(|| {
                let mut channel = EventChannel::<(CowStr, Event)>::new();
                let reader_id = channel.register_reader();
                res.insert(channel);
                Some(reader_id)
            });
    }
}
