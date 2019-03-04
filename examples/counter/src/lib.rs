use console_error_panic_hook::set_once as set_panic_hook;
use oak::events::{Event, EventListener, EventListeners, Handler};
use oak::dom::{VirtualElement, VirtualNode, ParentNode};
use oak::core::EventChannel;
use oak::app::{Stateful};
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

impl Handler<Msg> for Model {
    fn handle(&mut self, msg: &Msg) {
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
        .with_thread_local(oak::events::BrowserEventSystem {
            reader_id: None,
            receiver: None,
            phantom: std::marker::PhantomData::<(MouseEvent, Msg)>,
        })
        .with_thread_local(oak::events::LogSystem::<Msg>::new())
        .with_thread_local(oak::events::DispatchSystem {
            reader_id: None,
            phantom: std::marker::PhantomData::<(Msg, Model)>,
        })
        .with_thread_local(oak::app::StatefulSystem {
            phantom: std::marker::PhantomData::<(Msg, Model, VirtualNode)>,
        })
        .with_thread_local(oak::dom::BrowserNodeCreator)
        .with_thread_local(oak::dom::BrowserNodeMounter)
        .with_thread_local(oak::dom::BrowserNodeUpdater::default())
        .build();
    dispatcher.setup(&mut world.res);

    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let body = world
        .create_entity()
        .with(oak::dom::BrowserNode {
            node: document.body().unwrap().into(),
            is_mounted: true,
        })
        .build();

    {
        let increment_button = world
            .create_entity()
            .with(VirtualElement::new("button").into_node())
            .with(ParentNode(body))
            .with(EventListeners(vec![EventListener::new(
                "click",
                |e: &MouseEvent| Msg::Increment,
            )]))
            .build();
        world
            .create_entity()
            .with(VirtualNode::Text("+".to_owned()))
            .with(ParentNode(increment_button))
            .build();
    }

    {
        let div = world
            .create_entity()
            .with(VirtualElement::new("div").into_node())
            .with(ParentNode(body))
            .build();
        world
            .create_entity()
            .with(oak::app::Stateful {
                func: Box::new(|model: &Model| VirtualNode::Text(model.count.to_string())),
                phantom: std::marker::PhantomData::<Msg>,
            })
            .with(ParentNode(div))
            .build();
    }

    {
        let decrement_button = world
            .create_entity()
            .with(VirtualElement::new("button").into_node())
            .with(ParentNode(body))
            .with(EventListeners(vec![EventListener::new(
                "click",
                |e: &MouseEvent| Msg::Decrement,
            )]))
            .build();
        world
            .create_entity()
            .with(VirtualNode::Text("-".to_owned()))
            .with(ParentNode(decrement_button))
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
}
