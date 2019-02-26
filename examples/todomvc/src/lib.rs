use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use console_error_panic_hook::set_once as set_panic_hook;
use specs::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    console, Document, Event, EventTarget, HtmlInputElement, InputEvent, MouseEvent, Node,
};

use oak::browser::events::*;
use oak::markup::VirtualNode;
use oak::shrev::EventChannel;
use oak::state::State;

#[derive(Debug)]
struct Model {
    field: String,
    uid: u16,
    tasks: Vec<Task>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            field: "".to_owned(),
            uid: 3,
            tasks: vec![
                Task {
                    description: "Hello World".to_owned(),
                    completed: false,
                    edits: None,
                    id: 1,
                },
                Task {
                    description: "Foo Bar".to_owned(),
                    completed: true,
                    edits: None,
                    id: 2,
                },
            ],
        }
    }
}

#[derive(Default, Debug)]
struct Task {
    description: String,
    completed: bool,
    edits: Option<String>,
    id: u16,
}

#[derive(Debug)]
enum Msg {
    UpdateField(String),
    Add,
}

impl State<Msg> for Model {
    fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::UpdateField(field) => self.field = field.clone(),
            Msg::Add => (),
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
        .with_thread_local(oak::browser::events::BrowserEventSystem {
            reader_id: None,
            receiver: None,
            phantom: std::marker::PhantomData::<(InputEvent, Msg)>,
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
    let body_entity = world
        .create_entity()
        .with(oak::browser::BrowserNode {
            node: body.into(),
            is_mounted: true,
        })
        .build();

    let container = oak::markup::create_element(&mut world, "div", &[("class", "todomvc-wrapper")])
        .with(oak::markup::VirtualNodeParent(body_entity))
        .build();
    let section = oak::markup::create_element(&mut world, "section", &[("class", "todoapp")])
        .with(oak::markup::VirtualNodeParent(container))
        .build();
    {
        let header = oak::markup::create_element(&mut world, "header", &[("class", "header")])
            .with(oak::markup::VirtualNodeParent(section))
            .build();
        let h1 = oak::markup::create_element(&mut world, "h1", &[])
            .with(oak::markup::VirtualNodeParent(header))
            .build();
        oak::markup::create_text(&mut world, "todos")
            .with(oak::markup::VirtualNodeParent(h1))
            .build();
        oak::markup::create_element(
            &mut world,
            "input",
            &[
                ("class", "new-todo"),
                ("placeholder", "What needs to be done?"),
                ("autofocus", "true"),
                ("name", "newTodo"),
            ],
        )
        .with(oak::markup::VirtualNodeParent(header))
        .with(oak::browser::events::EventListeners(vec![(
            "input".to_owned(),
            Box::new(|e: &InputEvent| {
                Msg::UpdateField(
                    e.target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>()
                        .value(),
                )
            }),
        )]))
        .build();
    }

    build_task_list(&mut world, section, &Model::default());

    let footer = oak::markup::create_element(&mut world, "footer", &[("class", "info")])
        .with(oak::markup::VirtualNodeParent(container))
        .build();
    {
        let footer_p1 = oak::markup::create_element(&mut world, "p", &[])
            .with(oak::markup::VirtualNodeParent(footer))
            .build();
        oak::markup::create_text(&mut world, "Double-click to edit a todo")
            .with(oak::markup::VirtualNodeParent(footer_p1))
            .build();
    }

    {
        let footer_p2 = oak::markup::create_element(&mut world, "p", &[])
            .with(oak::markup::VirtualNodeParent(footer))
            .build();
        oak::markup::create_text(&mut world, "Written by ")
            .with(oak::markup::VirtualNodeParent(footer_p2))
            .build();
        let a = oak::markup::create_element(&mut world, "a", &[("href", "http://sagan.software")])
            .with(oak::markup::VirtualNodeParent(footer_p2))
            .build();
        oak::markup::create_text(&mut world, "sagan.software")
            .with(oak::markup::VirtualNodeParent(a))
            .build();
    }

    {
        let footer_p3 = oak::markup::create_element(&mut world, "p", &[])
            .with(oak::markup::VirtualNodeParent(footer))
            .build();
        oak::markup::create_text(&mut world, "Part of ")
            .with(oak::markup::VirtualNodeParent(footer_p3))
            .build();
        let a2 = oak::markup::create_element(&mut world, "a", &[("href", "http://todomvc.com")])
            .with(oak::markup::VirtualNodeParent(footer_p3))
            .build();
        oak::markup::create_text(&mut world, "TodoMVC")
            .with(oak::markup::VirtualNodeParent(a2))
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
            .write_resource::<EventChannel<BrowserEvent<MouseEvent>>>()
            .single_write(BrowserEvent {
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

    let world_rc2 = world_rc.clone();
    let dispatcher_rc2 = dispatcher_rc.clone();
    let cb = Closure::wrap(Box::new(move |event: InputEvent| {
        let mut world = world_rc2.borrow_mut();
        world
            .write_resource::<EventChannel<BrowserEvent<InputEvent>>>()
            .single_write(BrowserEvent {
                name: "input".to_owned(),
                event,
            });
        dispatcher_rc2.borrow_mut().dispatch(&world.res);
        world.maintain();
    }) as Box<dyn Fn(_)>);
    let et: &web_sys::EventTarget = &web_sys::window().unwrap().document().unwrap();
    et.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
}

fn build_task_list(world: &mut World, parent: Entity, model: &Model) {
    let section = oak::markup::create_element(world, "section", &[("class", "main")])
        .with(oak::markup::VirtualNodeParent(parent))
        .build();
    oak::markup::create_element(
        world,
        "input",
        &[
            ("class", "toggle-all"),
            ("id", "toggle-all"),
            ("type", "checkbox"),
            ("name", "toggle"),
        ],
    )
    .with(oak::markup::VirtualNodeParent(section))
    .build();
    {
        let label = oak::markup::create_element(world, "label", &[("for", "toggle-all")])
            .with(oak::markup::VirtualNodeParent(section))
            .build();
        oak::markup::create_text(world, "Mark all as completed")
            .with(oak::markup::VirtualNodeParent(label))
            .build();
    }

    let task_list = oak::markup::create_element(world, "ul", &[("class", "todo-list")])
        .with(oak::markup::VirtualNodeParent(section))
        .build();

    for task in &model.tasks {
        let li = oak::markup::create_element(world, "li", &[("class", "")])
            .with(oak::markup::VirtualNodeParent(task_list))
            .build();
        let div = oak::markup::create_element(world, "div", &[("class", "view")])
            .with(oak::markup::VirtualNodeParent(li))
            .build();
        oak::markup::create_element(world, "input", &[("class", "toggle"), ("type", "checkbox")])
            .with(oak::markup::VirtualNodeParent(div))
            .build();
        {
            let label = oak::markup::create_element(world, "label", &[])
                .with(oak::markup::VirtualNodeParent(div))
                .build();
            oak::markup::create_text(world, &task.description)
                .with(oak::markup::VirtualNodeParent(label))
                .build();
        }
        oak::markup::create_element(world, "button", &[("class", "destroy")])
            .with(oak::markup::VirtualNodeParent(div))
            .build();

        oak::markup::create_element(
            world,
            "input",
            &[
                ("class", "edit"),
                ("value", &task.description),
                ("name", "title"),
            ],
        )
        .with(oak::markup::VirtualNodeParent(li))
        .build();
    }
}
