use quote::quote;
use syn::parse_macro_input;
use syn_markup::*;

#[test]
fn open_el_basic() {
    let code = "<hello></hello>";
    let node = syn::parse_str::<Node>(code).unwrap();
    let el = node.open_element().unwrap();
    assert_eq!(el.open_tag.name, "hello");
    assert_eq!(el.open_tag.attributes.len(), 0);
    assert_eq!(el.children.len(), 0);
    assert_eq!(el.close_tag.name, "hello");
}

#[test]
fn open_el_wrong_closing_tag() {
    let code = "<hello></world>";
    let result = syn::parse_str::<Node>(code);
    assert!(result.is_err());
}

#[test]
fn open_el_no_closing_tag() {
    let code = "<hello>";
    let result = syn::parse_str::<Node>(code);
    assert!(result.is_err());
}

#[test]
fn open_el_with_attribute() {
    let code = "<div class=\"foo bar\"></div>";
    let node = syn::parse_str::<Node>(code).unwrap();
    let el = node.open_element().unwrap();
    assert_eq!(el.open_tag.name, "div");
    assert_eq!(el.open_tag.attributes.len(), 1);
    assert_eq!(el.open_tag.attributes[0].key, "class");
    assert_eq!(el.children.len(), 0);
    assert_eq!(el.close_tag.name, "div");
}

#[test]
fn open_el_with_attributes() {
    let code = "<div id=\"hello-world\" class=\"foo bar\"></div>";
    let node = syn::parse_str::<Node>(code).unwrap();
    let el = node.open_element().unwrap();
    assert_eq!(el.open_tag.name, "div");
    assert_eq!(el.open_tag.attributes.len(), 2);
    assert_eq!(el.open_tag.attributes[0].key, "id");
    assert_eq!(el.open_tag.attributes[1].key, "class");
    assert_eq!(el.children.len(), 0);
    assert_eq!(el.close_tag.name, "div");
}

#[test]
fn open_el_with_children() {
    let code = "<div><span></span><input/>Test{ test }</div>";
    let node = syn::parse_str::<Node>(code).unwrap();
    let el = node.open_element().unwrap();
    assert_eq!(el.children.len(), 4);
}

#[test]
fn open_el_with_nested_children() {
    let code = "<h1><span>Hello</span><strong>World!</strong></h1>";
    let node = syn::parse_str::<Node>(code).unwrap();
    let el = node.open_element().unwrap();
    assert_eq!(el.children.len(), 2);

    let child_0 = el.children[0].open_element().unwrap();
    assert_eq!(child_0.open_tag.name, "span");
    assert_eq!(child_0.children.len(), 1);

    let child_1 = el.children[1].open_element().unwrap();
    assert_eq!(child_1.open_tag.name, "strong");
    assert_eq!(child_1.children.len(), 1);
}

#[test]
fn void_tag_basic() {
    let code = "<input/>";
    let node = syn::parse_str::<Node>(code).unwrap();
    let el = node.void_element().unwrap();
    assert_eq!(el.name, "input");
    assert_eq!(el.attributes.len(), 0);
}

#[test]
fn void_tag_with_type_attribute() {
    let code = "<input type=\"password\" />";
    let node = syn::parse_str::<Node>(code).unwrap();
    let el = node.void_element().unwrap();
    assert_eq!(el.name, "input");
    assert_eq!(el.attributes.len(), 1);
    assert_eq!(el.attributes[0].key, "type");
}

#[test]
fn text() {
    let code = "Hello World!";
    let node = syn::parse_str::<Node>(code).unwrap();
    let text = node.text().unwrap();
    assert_eq!(text.text, "Hello World!");
}

#[test]
fn braced() {
    let code = "{ testing }";
    let node = syn::parse_str::<Node>(code).unwrap();
    let braced = node.braced().unwrap();
}

// #[test]
// fn tags_with_spaces() {
//     let code = "<div> <span>Hello</span> </div>";
//     let node = syn::parse_str::<Node>(code).unwrap();
//     let el = node.open_element().unwrap();
//     assert_eq!(el.children.len(), 3);
//     assert_eq!(
//         el.children[0],
//         Node::Text(Text {
//             text: " ".to_owned()
//         })
//     );
//     assert_eq!(
//         el.children[2],
//         Node::Text(Text {
//             text: " ".to_owned()
//         })
//     );
// }

#[test]
fn basic_text() {
    let code = "Hello World!";
    let markup = syn::parse_str::<Markup>(code).unwrap();
    assert_eq!(markup.nodes.len(), 1);
    let text = markup.nodes[0].text().unwrap();
    assert_eq!(text.text, "Hello World!");
}

#[test]
fn text_with_trailing_space() {
    let code = "Hello { world }!";
    let markup = syn::parse_str::<Markup>(code).unwrap();
    assert_eq!(markup.nodes.len(), 3);
    let text = markup.nodes[0].text().unwrap();
    assert_eq!(text.text, "Hello ");
}

#[test]
fn text_inside_tag() {
    let code = "<h1>Hello World!</h1>";
    let markup = syn::parse_str::<Markup>(code).unwrap();
    let text = markup.nodes[0].open_element().unwrap().children[0]
        .text()
        .unwrap();
    assert_eq!(text.text, "Hello World!");
}
