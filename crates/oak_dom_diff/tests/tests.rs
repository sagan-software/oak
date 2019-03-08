use oak_dom_diff::{diff, Patch};
use oak_dom_node::{html::*, Node, text};
use std::collections::BTreeMap;

pub fn test<A, B>(desc: &str, old: A, new: B, expected: Vec<Patch<'_>>)
where
    A: Into<Node>,
    B: Into<Node>,
{
    // ex: vec![Patch::Replace(0, &html! { <strong></strong> })],
    let old_node = old.into();
    let new_node = new.into();
    let patches = diff(&old_node, &new_node);
    assert_eq!(patches, expected, "{}", desc);
}

#[test]
fn replace_node() {
    test(
        "Replace the root if the tag changed",
        div(),
        span(),
        vec![Patch::Replace(0, &span().into())],
    );
    test(
        "Replace a child node",
        div().push(b()),
        div().push(strong()),
        vec![Patch::Replace(1, &strong().into())],
    );
    test(
        "Replace node with a child",
        div().push(b().push("A")).push(b()),
        div().push(i().push("A")).push(i()),
        vec![Patch::Replace(1, &i().push("A").into()), Patch::Replace(3, &i().into())],
    );
}

#[test]
fn add_children() {
    test(
        "Added a new node to the root node",
        div().push(b()),
        div().push(b()).push(i()),
        vec![Patch::AppendChildren(0, vec![&i().into()])],
    );
}

#[test]
fn remove_nodes() {
    test(
        "Remove all child nodes at and after child sibling index 1",
        div().push(b()).push(span()),
        div(),
        vec![Patch::TruncateChildren(0, 0)],
    );
    test(
        "Remove a child and a grandchild node",
        div().push(span().push(b()).push(i())).push(strong()),
        div().push(span().push(b())),
        vec![Patch::TruncateChildren(0, 1), Patch::TruncateChildren(1, 1)],
    );
    test(
        "Removing child and change next node after parent",
        div().push(b().push(i()).push(i())).push(b()),
        div().push(b().push(i())).push(i()),
        vec![Patch::TruncateChildren(1, 1), Patch::Replace(4, &i().into())], //required to check correct index
    );
}

#[test]
fn add_attributes() {
    let mut attributes = BTreeMap::new();
    attributes.insert("id", "hello");

    test(
        "Add attributes",
        div(),
        div().set(id("hello")),
        vec![Patch::SetAttributes(0, attributes.clone())],
    );

    test(
        "Change attribute",
        div().set(id("foobar")),
        div().set(id("hello")),
        vec![Patch::SetAttributes(0, attributes)],
    );
}

#[test]
fn remove_attributes() {
    test(
        "Add attributes",
        div().set(id("hey-there")),
        div(),
        vec![Patch::RemoveAttributes(0, vec!["id"])],
    );
}

#[test]
fn change_attribute() {
    let mut attributes = BTreeMap::new();
    attributes.insert("id", "changed");

    test(
        "Add attributes",
        div().set(id("hey-there")),
        div().set(id("changed")),
        vec![Patch::SetAttributes(0, attributes)],
    );
}

#[test]
fn replace_text_node() {
    test(
        "Replace text node",
        text("Old"),
        text("New"),
        vec![Patch::ChangeText(0, "New")],
    );
}
