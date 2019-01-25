use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use console_error_panic_hook::set_once as set_panic_hook;
use specs::prelude::*;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{console, Document, Event, EventTarget, Node};

use oak::browser::events::*;
use oak::shrev::EventChannel;
use oak::state::{State, StateData, Trans};

#[derive(Default)]
struct Model {
    count: u16,
}

enum Msg {
    Increment,
    Decrement,
}

impl State<Model, Msg> for Model {
    fn handle_msg(&mut self, _data: StateData<'_, Model>, msg: Msg) -> Trans<Model, Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
        Trans::None
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();

    let mut world = World::new();
    world.add_resource(Model::default());
    world.add_resource(EventChannel::<MouseEvent>::new());

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(oak::browser::BrowserNodeCreator)
        .with_thread_local(oak::browser::BrowserNodeMounter)
        .with_thread_local(oak::browser::events::EventSystem {
            reader_id: None,
            phantom: std::marker::PhantomData::<(MouseEvent, Msg)>,
            receiver: None,
        })
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
        .with(oak::browser::events::EventListener {
            name: "click".to_owned(),
            func: Box::new(|e: &MouseEvent| {
                web_sys::console::log_1(&JsValue::from("INCREMENT"));
                Msg::Increment
            }),
        })
        .build();
    oak::markup::create_text(&mut world, "+")
        .with(oak::markup::VirtualNodeParent(increment_button))
        .build();

    oak::markup::create_text(&mut world, "0")
        .with(oak::markup::VirtualNodeParent(body_entity))
        .build();

    let decrement_button = oak::markup::create_element(
        &mut world, "button", &[],
    )
        .with(oak::markup::VirtualNodeParent(body_entity))
        .with(oak::browser::events::EventListener {
            name: "click".to_owned(),
            func: Box::new(|e: &MouseEvent| {
                web_sys::console::log_1(&JsValue::from("DECREMENT"));
                Msg::Increment
            }),
        })
        .build();
    oak::markup::create_text(&mut world, "-")
        .with(oak::markup::VirtualNodeParent(decrement_button))
        .build();

    dispatcher.dispatch(&world.res);
    world.maintain();

//    let world_rc = Rc::new(RefCell::new(world));
//    let dispatcher_rc = Rc::new(RefCell::new(dispatcher));
//
//    let world_rc2 = world_rc.clone();
//    let dispatcher_rc2 = dispatcher_rc.clone();
//    let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
//        web_sys::console::log_2(&JsValue::from("Clicked?"), &e);
//        let mut world = world_rc2.borrow_mut();
//        //
//        world
//            .write_resource::<MouseEvents>()
//            .channel
//            .single_write(MouseEvent(e));
//        //
//        dispatcher_rc2.borrow_mut().dispatch(&world.res);
//        //
//        world.maintain();
//    }) as Box<dyn Fn(_)>);
//    let et: &web_sys::EventTarget = &web_sys::window().unwrap().document().unwrap();
//    et
//        .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
//        .unwrap();
//    cb.forget();
}
