mod sys;

use crate::sys::*;
use oak::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn main() -> AppResult {
    App::view("Hello World!").mount("#App")
}

enum Msg {
    Update,
    Finish,
}

// fn init() -> AppState {
//     sys::init("oak", "0.1");
// }

fn view(app: &AppState) -> HtmlElement<()> {
    let inner = match app.location().as_str() {
        "table" => view_table(&app.table().unchecked_ref::<sys::TableState>()),
        "anim" => view_anim(&app.anim().unchecked_ref::<sys::AnimState>()),
        "tree" => view_tree(&app.tree().unchecked_ref::<sys::TreeState>()),
        _ => div(),
    };
    div().set(class("Main")).push(inner)
}

fn view_table(state: &TableState) -> HtmlElement<()> {
    table().set(class("Table")).push_iter(
        state
            .items()
            .iter()
            .map(|v| view_table_row(&v.unchecked_ref::<TableItemState>())),
    )
}

fn view_table_row(state: &TableItemState) -> HtmlElement<()> {
    let row_id = state.id().to_string();
    let pound_row_id = "#".to_string() + row_id.as_str();
    tr().set(class("TableRow"))
        .set(id(&row_id))
        .push(view_table_cell(&pound_row_id))
        .push_iter(state.props().iter().map(|v| {
            let string: String = v.unchecked_ref::<js_sys::JsString>().into();
            view_table_cell(&string)
        }))
}

fn view_table_cell(text: &str) -> HtmlElement<()> {
    td().set(class("TableCell"))
        .set(data("text", text))
        .push(text)
}

fn view_anim(state: &AnimState) -> HtmlElement<()> {
    div().set(class("Anim")).push_iter(
        state
            .items()
            .iter()
            .map(|v| view_anim_box(v.unchecked_ref::<AnimBoxState>())),
    )
}

fn view_anim_box(state: &AnimBoxState) -> HtmlElement<()> {
    let mut styles = "border-radius:".to_string();
    let border_radius = state.time() % 10.0;
    styles.push_str(&border_radius.to_string());
    styles.push_str("px;background:rgba(0,0,0,");
    let alpha = border_radius / 10.0 + 0.5;
    styles.push_str(&alpha.to_string());
    styles.push(')');
    div()
        .set(class("AnimBox"))
        .set(data("id", state.id()))
        .set(style(styles))
}

fn view_tree(state: &TreeState) -> HtmlElement<()> {
    let root = state.root();
    div()
        .set(class("Tree"))
        .push(view_tree_node(root.unchecked_ref::<TreeNodeState>()))
}

fn view_tree_node(state: &TreeNodeState) -> HtmlElement<()> {
    let node = ul().set(class("TreeNode"));
    match state.children() {
        Some(children) => node.push_iter(children.iter().map(|v| {
            let child = v.unchecked_ref::<TreeNodeState>();
            if child.container() {
                view_tree_node(child)
            } else {
                view_tree_leaf(child)
            }
        })),
        None => node,
    }
}

fn view_tree_leaf(state: &TreeNodeState) -> HtmlElement<()> {
    li().set(class("TreeLeaf")).push(state.id())
}
