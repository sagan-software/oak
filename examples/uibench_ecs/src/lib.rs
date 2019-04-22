#[macro_use]
mod core;
mod dom;
mod input;
mod uibench;

use crate::core::Document;
use crate::dom::{
    compile_template, CustomElementTemplates, HtmlTemplateElement, Node, NodeParent, NodeSystem,
};
use crate::input::{EventDelgationSystem, EventListener};
use console_error_panic_hook::set_once as set_panic_hook;
use specs::prelude::*;
use specs_hierarchy::HierarchySystem;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

    dispatcher.dispatch(&world.res);
    world.maintain();

    std::mem::forget(world);

    Ok(())
}

const TABLE_CELL_VIEW: &str = "<td class=TableCell></td>";

fn create_table_cell(world: &mut World, parent: Entity, data: &str) {
    let el = compile_template(world, TABLE_CELL_VIEW);
    el.set_text_content(Some(data));
    el.set_attribute("data-text", data);
    world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .with(EventListener {
            name: "click".into(),
            func: {
                let text = data.to_string();
                Box::new(move |_| {
                    web_sys::console::log_2(&JsValue::from_str("Click"), &JsValue::from_str(&text))
                })
            },
        })
        .build();
}

const TABLE_ROW_VIEW: &str = "<tr></tr>";

fn create_table_row(world: &mut World, parent: Entity, data: &uibench::TableItemState) {
    let el = compile_template(world, TABLE_ROW_VIEW);
    if data.active() {
        el.set_class_name("active");
    }
    let id = data.id().to_string();
    el.set_id(&id);

    let entity = world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();

    let mut pound_id = String::from("#");
    pound_id.push_str(&id);
    create_table_cell(world, entity, &pound_id);

    for prop in data.props().iter() {
        let value: String = prop.unchecked_ref::<js_sys::JsString>().into();
        create_table_cell(world, entity, &value);
    }
}

const TABLE_VIEW: &str = "<table class=Table><tbody></tbody></table>";

fn create_table(world: &mut World, parent: Entity, data: &uibench::TableState) {
    let el = compile_template(world, TABLE_VIEW);

    let tbody_el = el.first_element_child().unwrap();
    let tbody_entity = world.create_entity().with(Node(tbody_el.into())).build();

    for item in data.items().iter() {
        create_table_row(
            world,
            tbody_entity,
            item.unchecked_ref::<uibench::TableItemState>(),
        );
    }

    world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
}

const ANIM_BOX_VIEW: &str = "<div class=AnimBox></div>";

fn create_anim_box(world: &mut World, parent: Entity, data: &uibench::AnimBoxState) {
    let el = compile_template(world, ANIM_BOX_VIEW);
    let style = el.unchecked_ref::<web_sys::HtmlElement>().style();

    let border_radius_value = data.time() % 10.0;
    let mut border_radius_string = border_radius_value.to_string();
    border_radius_string.push_str("px");
    style.set_property("border-radius", &border_radius_string);

    let alpha_value = border_radius_value / 10.0 + 0.5;
    let mut alpha_string = "rgba(0,0,0,".to_string();
    alpha_string.push_str(&alpha_value.to_string());
    style.set_property("background", &alpha_string);

    el.set_attribute("data-id", &data.id().to_string());
    world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
}

const ANIM_VIEW: &str = "<div class=Anim></div>";

fn create_anim(world: &mut World, parent: Entity, data: &uibench::AnimState) {
    let el = compile_template(world, ANIM_VIEW);
    let entity = world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
    for item in data.items().iter() {
        create_anim_box(world, entity, item.unchecked_ref::<uibench::AnimBoxState>());
    }
}

const TREE_LEAF_VIEW: &str = "<li class=TreeLeaf></li>";

fn create_tree_leaf(world: &mut World, parent: Entity, data: &uibench::TreeNodeState) {
    let el = compile_template(world, TREE_LEAF_VIEW);
    el.set_text_content(Some(&data.id().to_string()));
    world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
}

const TREE_NODE_VIEW: &str = "<ul class=TreeNode></ul>";

fn create_tree_node(world: &mut World, parent: Entity, data: &uibench::TreeNodeState) {
    let el = compile_template(world, TREE_NODE_VIEW);
    let entity = world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
    if let Some(children) = &data.children() {
        for value in children.iter() {
            let child = value.unchecked_ref::<uibench::TreeNodeState>();
            if child.container() {
                create_tree_node(world, entity, child);
            } else {
                create_tree_leaf(world, entity, child);
            }
        }
    }
}

const TREE_VIEW: &str = "<div class=Tree></div>";

fn create_tree(world: &mut World, parent: Entity, data: &uibench::TreeState) {
    let el = compile_template(world, TREE_VIEW);
    let entity = world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
    create_tree_node(
        world,
        entity,
        data.root().unchecked_ref::<uibench::TreeNodeState>(),
    );
}

const MAIN_VIEW: &str = "<div class=Main></div>";

fn create_main(world: &mut World, parent: Entity, data: &uibench::AppState) {
    let el = compile_template(world, MAIN_VIEW);
    let entity = world
        .create_entity()
        .with(Node(el.into()))
        .with(NodeParent(parent))
        .build();
    match data.location().as_str() {
        "table" => create_table(
            world,
            entity,
            data.table().unchecked_ref::<uibench::TableState>(),
        ),
        "anim" => create_anim(
            world,
            entity,
            data.table().unchecked_ref::<uibench::AnimState>(),
        ),
        "tree" => create_tree(
            world,
            entity,
            data.table().unchecked_ref::<uibench::TreeState>(),
        ),
        _ => (),
    }
}
