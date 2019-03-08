#![recursion_limit = "128"]
use oak_dom_node::{VirtualChildren, VirtualElement, VirtualNode};
use oak_macros::html;
use std::collections::BTreeMap;

struct HtmlMacroTest<'a> {
    desc: &'a str,
    generated: VirtualNode,
    expected: VirtualNode,
}

impl<'a> HtmlMacroTest<'a> {
    /// Ensure that the generated and the expected virtual node are equal.
    fn test(self) {
        assert_eq!(self.generated, self.expected, "{}", self.desc);
    }
}

#[test]
fn empty_div() {
    HtmlMacroTest {
        desc: "Empty div",
        generated: html! { <div></div> },
        expected: VirtualElement::open("div").into(),
    }
    .test();
}

#[test]
fn one_attr() {
    let mut attributes = BTreeMap::new();
    attributes.insert("id".to_string(), "hello-world".to_string());
    let mut expected = VirtualElement::open("div");
    expected.attributes = attributes;

    HtmlMacroTest {
        desc: "One attribute",
        generated: html! { <div id="hello-world"></div> },
        expected: expected.into(),
    }
    .test();
}

// #[test]
// fn event() {
//     HtmlMacroTest {
//         desc: "Events are ignored in non wasm-32 targets",
//         generated: html! {
//             <div onclick=|_: u8|{}></div>
//         },
//         expected: html! {<div></div>},
//     }
//     .test();
// }

#[test]
fn child_node() {
    let mut expected = VirtualElement::open("div");
    expected.children = VirtualChildren::Nodes(vec![VirtualElement::open("span").into()]);

    HtmlMacroTest {
        desc: "Child node",
        generated: html! { <div><span></span></div> },
        expected: expected.into(),
    }
    .test();
}

#[test]
fn sibling_child_nodes() {
    let mut expected = VirtualElement::open("div");
    expected.children = VirtualChildren::Nodes(vec![
        VirtualElement::open("span").into(),
        VirtualElement::open("b").into(),
    ]);

    HtmlMacroTest {
        desc: "Sibling child nodes",
        generated: html! { <div><span></span><b></b></div> },
        expected: expected.into(),
    }
    .test();
}

#[test]
fn three_nodes_deep() {
    let mut child = VirtualElement::open("span");
    child.children = VirtualChildren::Nodes(vec![VirtualElement::open("b").into()]);

    let mut expected = VirtualElement::open("div");
    expected.children = VirtualChildren::Nodes(vec![child.into()]);

    HtmlMacroTest {
        desc: "Nested 3 nodes deep",
        generated: html! { <div><span><b></b></span></div> },
        expected: expected.into(),
    }
    .test()
}

#[test]
fn sibling_text_nodes() {
    let mut expected = VirtualElement::open("div");
    expected.children = VirtualChildren::Nodes(vec!["This is a text node".into()]);

    HtmlMacroTest {
        desc: "Nested text node",
        generated: html! { <div>This is a text node</div> },
        expected: expected.into(),
    }
    .test();
}

#[test]
fn nested_macro() {
    let child_2 = html! { <b></b> };

    let mut expected = VirtualElement::open("div");
    expected.children = VirtualChildren::Nodes(vec![
        VirtualElement::open("span").into(),
        VirtualElement::open("b").into(),
    ]);

    HtmlMacroTest {
        desc: "Nested macros",
        generated: html! {
          <div>
            { html! { <span></span> } }
            { child_2 }
          </div>
        },
        expected: expected.into(),
    }
    .test();
}

#[test]
fn block_root() {
    let em = html! { <em></em> };

    let expected = VirtualElement::open("em");

    HtmlMacroTest {
        desc: "Block root node",
        generated: html! {
            { em }
        },
        expected: expected.into(),
    }
    .test();
}

#[test]
fn text_next_to_block() {
    let child = html! { <ul></ul> };

    let mut expected = VirtualElement::open("div");
    expected.children = VirtualChildren::Nodes(vec![
        "A bit of text".into(),
        VirtualElement::open("ul").into(),
    ]);

    HtmlMacroTest {
        desc: "Text node next to a block",
        generated: html! {
          <div>
            A bit of text
            { child }
          </div>
        },
        expected: expected.into(),
    }
    .test();
}

#[test]
fn punctuation_comma() {
    let text = "Hello, World";

    HtmlMacroTest {
        desc: "Comma",
        generated: html! { Hello, World },
        expected: text.into(),
    }
    .test()
}

#[test]
fn punctuation_exclamation() {
    let text = "Hello World!";

    HtmlMacroTest {
        desc: "Exclamation point",
        generated: html! { Hello World! },
        expected: text.into(),
    }
    .test()
}

#[test]
fn punctuation_period() {
    let text = "Hello.";

    HtmlMacroTest {
        desc: "Period",
        generated: html! { Hello. },
        expected: text.into(),
    }
    .test()
}

// TODO
// #[test]
// fn vec_of_nodes() {
//     let children = vec![html! { <div> </div> }, html! { <strong> </strong> }];

//     let mut expected = VirtualElement::open("div");
//     expected.children = VirtualChildren::Nodes(vec![
//         VirtualElement::open("div").into(),
//         VirtualElement::open("strong").into(),
//     ]);

//     HtmlMacroTest {
//         desc: "Vec of nodes",
//         generated: html! { <div> { children } </div> },
//         expected: expected.into(),
//     }
//     .test();
// }

#[test]
fn text_root_node() {
    HtmlMacroTest {
        desc: "Text as root node",
        generated: html! { some text },
        expected: "some text".into(),
    }
    .test()
}

/// Just make sure that this compiles since type is a keyword
#[test]
fn type_attribute() {
    html! { <link rel="stylesheet" type="text/css" href="/app.css" /> };
}

// Verify that all of our self closing tags work as both.
// Self closing tags can be written as either <tag> and <tag />
#[test]
fn self_closing_tag() {
    let mut expected = VirtualElement::open("div");
    let children = vec![
        "area", "base", "br", "col", "hr", "img", "input", "link", "meta", "param", "command",
        "keygen", "source",
    ]
    .into_iter()
    .map(|tag| VirtualElement::void(tag).into())
    .collect::<Vec<VirtualNode>>();
    expected.children = VirtualChildren::Nodes(children);

    let desc = &format!("Self closing tag without baskslash");
    HtmlMacroTest {
        desc,
        generated: html! {
            <div>
                <area/> <base/> <br/> <col/> <hr/> <img/> <input/> <link/> <meta/> <param/> <command/>
                <keygen/> <source/>
            </div>
        },
        expected: expected.into(),
    }
    .test();

    let desc = &format!("Self closing tag with backslash");
    HtmlMacroTest {
        desc,
        generated: html! {
            <br />
        },
        expected: VirtualElement::void("br").into(),
    }
    .test();
}
