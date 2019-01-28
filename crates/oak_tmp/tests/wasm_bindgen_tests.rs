use oak::html::{attributes::class, div, text, Html};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// fn simple_text_nodes() {
//     let html: Html<()> = text("Test!");
//     let out = oak::browser::render(&html);
//     assert_eq!(out.is_ok(), true);
//     assert_eq!(out.unwrap().text_content(), Some("Test!".to_owned()));
// }

// #[wasm_bindgen_test]
// fn simple_tag_nodes() {
//     let html: Html<()> = div(vec![class("hello world")], vec![text("Foobar")]);
//     let out = oak::browser::render(&html);
//     assert_eq!(out.is_ok(), true);
//     assert_eq!(out.unwrap().text_content(), Some("Foobar".to_owned()));
// }
