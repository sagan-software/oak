use console_error_panic_hook::set_once as set_panic_hook;
use oak::events::{Event, EventListener, EventListeners};
use oak::markup::{Element, Node, NodeParent};
use oak::shrev::EventChannel;
use oak::state::{State, Stateful};
use specs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::MouseEvent;

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
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(oak::browser::events::BrowserEventSystem {
            reader_id: None,
            receiver: None,
            phantom: std::marker::PhantomData::<(MouseEvent, Msg)>,
        })
        .with_thread_local(oak::events::EventLogger::<Msg>::new())
        .with_thread_local(oak::state::StateUpdater {
            reader_id: None,
            phantom: std::marker::PhantomData::<(Msg, Model)>,
        })
        .with_thread_local(oak::state::StatefulSystem {
            phantom: std::marker::PhantomData::<(Msg, Model, Node)>,
        })
        .with_thread_local(oak::browser::BrowserNodeCreator)
        .with_thread_local(oak::browser::BrowserNodeMounter)
        .with_thread_local(oak::browser::BrowserNodeUpdater::default())
        .build();
    dispatcher.setup(&mut world.res);

    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let body = world
        .create_entity()
        .with(oak::browser::BrowserNode {
            node: document.body().unwrap().into(),
            is_mounted: true,
        })
        .build();

    {
        let increment_button = world
            .create_entity()
            .with(Element::new("button").into_node())
            .with(NodeParent(body))
            .with(EventListeners(vec![EventListener::new(
                "click",
                |e: &MouseEvent| Msg::Increment,
            )]))
            .build();
        world
            .create_entity()
            .with(Node::Text("+".to_owned()))
            .with(NodeParent(increment_button))
            .build();
    }

    {
        let div = world
            .create_entity()
            .with(Element::new("div").into_node())
            .with(NodeParent(body))
            .build();
        world
            .create_entity()
            .with(oak::state::Stateful {
                func: Box::new(|model: &Model| Node::Text(model.count.to_string())),
                phantom: std::marker::PhantomData::<Msg>,
            })
            .with(NodeParent(div))
            .build();
    }

    {
        let decrement_button = world
            .create_entity()
            .with(Element::new("button").into_node())
            .with(NodeParent(body))
            .with(EventListeners(vec![EventListener::new(
                "click",
                |e: &MouseEvent| Msg::Decrement,
            )]))
            .build();
        world
            .create_entity()
            .with(Node::Text("-".to_owned()))
            .with(NodeParent(decrement_button))
            .build();
    }

    dispatcher.dispatch(&world.res);
    world.maintain();

    let world_rc = Rc::new(RefCell::new(world));
    let dispatcher_rc = Rc::new(RefCell::new(dispatcher));

    let world_rc2 = world_rc.clone();
    let dispatcher_rc2 = dispatcher_rc.clone();
    let cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut world = world_rc2.borrow_mut();
        world
            .write_resource::<EventChannel<Event<MouseEvent>>>()
            .single_write(Event {
                name: "click".to_owned(),
                event,
            });
        dispatcher_rc2.borrow_mut().dispatch(&world.res);
        world.maintain();
    }) as Box<dyn Fn(_)>);
    let et: &web_sys::EventTarget = &web_sys::window().unwrap().document().unwrap();
    et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();

    let world_rc3 = world_rc.clone();
    let dispatcher_rc3 = dispatcher_rc.clone();
    let cb2 = Closure::wrap(Box::new(move || {
        web_sys::console::log_1(&JsValue::from("Balls"));
        // let mut world = world_rc3.borrow_mut();
        // world
        //     .write_resource::<EventChannel<String>>()
        //     .single_write("testing".to_owned());
        // dispatcher_rc3.borrow_mut().dispatch(&world.res);
        // world.maintain();
    }) as Box<dyn Fn()>);
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(cb2.as_ref().unchecked_ref(), 1_000)
        .unwrap();
    cb2.forget();

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
