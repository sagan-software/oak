use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use console_error_panic_hook::set_once as set_panic_hook;
use specs::prelude::*;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{console, Document, Event, EventTarget, MouseEvent, Node};

use oak::browser::events::*;
use oak::markup::VirtualNode;
use oak::shrev::EventChannel;
use oak::state::State;

#[derive(Default, Debug)]
struct Model {
    count: i16,
}

#[derive(Debug)]
enum Msg {
    Increment,
    Decrement,
}

impl State<Msg> for Model {
    fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    set_panic_hook();

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(oak::browser::events::BrowserEventSystem {
            reader_id: None,
            receiver: None,
            phantom: std::marker::PhantomData::<(MouseEvent, Msg)>,
        })
        .with_thread_local(oak::browser::events::EventLogger {
            reader_id: None,
            phantom: std::marker::PhantomData::<Msg>,
        })
        .with_thread_local(oak::state::StateUpdater {
            reader_id: None,
            phantom: std::marker::PhantomData::<(Msg, Model)>,
        })
        .with_thread_local(oak::state::StatefulSystem {
            phantom: std::marker::PhantomData::<(Msg, Model, VirtualNode)>,
        })
        .with_thread_local(oak::browser::BrowserNodeCreator)
        .with_thread_local(oak::browser::BrowserNodeMounter)
        .with_thread_local(oak::browser::BrowserNodeUpdater::default())
        .build();
    dispatcher.setup(&mut world.res);

    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let body = document.body().unwrap();
    //
    let body_entity = world
        .create_entity()
        .with(oak::browser::BrowserNode {
            node: body.into(),
            is_mounted: true,
        })
        .build();

    let increment_button = oak::markup::create_element(
        &mut world, "button", &[],
    )
        .with(oak::markup::VirtualNodeParent(body_entity))
        .with(oak::browser::events::EventListeners(vec![
            (
                "click".to_owned(),
                Box::new(|e: &MouseEvent| {
                    Msg::Increment
                }),
            ),
//            (
//                "mouseover".to_owned(),
//                Box::new(|e: &MouseEvent| {
//                    Msg::Increment
//                }),
//            )
        ]))
        .build();
    oak::markup::create_text(&mut world, "+")
        .with(oak::markup::VirtualNodeParent(increment_button))
        .build();

    world.create_entity()
        .with(oak::state::Stateful {
            func: Box::new(|model: &Model| {
                oak::markup::VirtualNode::Text(model.count.to_string())
            }),
            phantom: std::marker::PhantomData::<Msg>,
        })
        .with(oak::markup::VirtualNodeParent(body_entity))
        .build();

    let decrement_button = oak::markup::create_element(
        &mut world, "button", &[],
    )
        .with(oak::markup::VirtualNodeParent(body_entity))
        .with(oak::browser::events::EventListeners(vec![
            (
                "click".to_owned(),
                Box::new(|e: &MouseEvent| {
                    Msg::Decrement
                }),
            )
        ]))
        .build();
    oak::markup::create_text(&mut world, "-")
        .with(oak::markup::VirtualNodeParent(decrement_button))
        .build();

    dispatcher.dispatch(&world.res);
    world.maintain();

    let world_rc = Rc::new(RefCell::new(world));
    let dispatcher_rc = Rc::new(RefCell::new(dispatcher));

    let world_rc2 = world_rc.clone();
    let dispatcher_rc2 = dispatcher_rc.clone();
    let cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut world = world_rc2.borrow_mut();
        world
            .write_resource::<EventChannel<BrowserEvent<MouseEvent>>>()
            .single_write(BrowserEvent {
                name: "click".to_owned(),
                event,
            });
        dispatcher_rc2.borrow_mut().dispatch(&world.res);
        world.maintain();
    }) as Box<dyn Fn(_)>);
    let et: &web_sys::EventTarget = &web_sys::window().unwrap().document().unwrap();
    et
        .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();


//    let world_rc2 = world_rc.clone();
//    let dispatcher_rc2 = dispatcher_rc.clone();
//    let cb = Closure::wrap(Box::new(move |event: MouseEvent| {
//        let mut world = world_rc2.borrow_mut();
//        world
//            .write_resource::<EventChannel<BrowserEvent<MouseEvent>>>()
//            .single_write(BrowserEvent {
//                name: "mouseover".to_owned(),
//                event,
//            });
//        dispatcher_rc2.borrow_mut().dispatch(&world.res);
//        world.maintain();
//    }) as Box<dyn Fn(_)>);
//    let et: &web_sys::EventTarget = &web_sys::window().unwrap().document().unwrap();
//    et
//        .add_event_listener_with_callback("mouseover", cb.as_ref().unchecked_ref())
//        .unwrap();
//    cb.forget();
}
