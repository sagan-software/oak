#[macro_use]
mod core;
mod dom;
mod input;

use crate::core::Document;
use crate::dom::{CustomElementTemplates, HtmlTemplateElement, Node, NodeParent, NodeSystem};
use crate::input::{EventDelgationSystem, EventListener};
use console_error_panic_hook::set_once as set_panic_hook;
use specs::prelude::*;
use specs_hierarchy::HierarchySystem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    let mut world = World::new();
    world.add_resource(Document::default());
    world.add_resource(HtmlTemplateElement::default());
    world.add_resource(CustomElementTemplates::default());

    // world.register::<crate::dom::Document>();

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(HierarchySystem::<NodeParent>::new())
        .with_thread_local(NodeSystem::default())
        .with_thread_local(EventDelgationSystem::default())
        .build();

    dispatcher.setup(&mut world.res);

    let document = web_sys::window().unwrap().document().unwrap();
    let body = world
        .create_entity()
        .with(Node(document.body().unwrap().into()))
        .build();

    let g1 = create_greeting(&mut world, body);
    // let g2 = create_greeting(&mut world, body);
    create_counter(&mut world, body);
    create_counter(&mut world, body);

    dispatcher.dispatch(&world.res);
    world.maintain();

    world.delete_entities(&[g1]).unwrap();
    let g2 = create_greeting(&mut world, body);
    let g3 = create_greeting(&mut world, body);

    dispatcher.dispatch(&world.res);
    world.maintain();

    std::mem::forget(world);

    Ok(())
}

fn create_greeting(world: &mut World, parent: Entity) -> Entity {
    let el = {
        let document = world.read_resource::<Document>();
        let el = document.create_element("h1").unwrap();
        el.set_text_content(Some("Hello World"));
        el
    };
    world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build()
}

const COUNTER_TEMPLATE: &str = r#"
<div>
    <button>+</button>
    <div></div>
    <button>-</button>
</div>
"#;

fn create_counter(world: &mut World, parent: Entity) -> Entity {
    let el = crate::dom::compile_template(world, COUNTER_TEMPLATE);
    let increment_el = el.first_element_child().unwrap();
    let count_el = increment_el.next_element_sibling().unwrap();
    let decrement_el = count_el.next_element_sibling().unwrap();
    count_el.set_text_content(Some("0"));
    let root = world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
    world
        .create_entity()
        .with(Node(increment_el.into()))
        .with(NodeParent(root))
        .with(EventListener {
            name: "click".into(),
            func: Box::new(|_| log::info!("INCREMENT!")),
        })
        .build();
    world
        .create_entity()
        .with(Node(count_el.into()))
        .with(NodeParent(root))
        .build();
    world
        .create_entity()
        .with(Node(decrement_el.into()))
        .with(NodeParent(root))
        .build();
    root
}
