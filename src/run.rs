use oak_core::web_sys;
use oak_vdom::Text;

pub fn run<S, I>(selector: S, init: I)
where
    S: Into<Text>,
{
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let selector = selector.into();
    let element = document.query_selector(selector).unwrap();
    element.set_text_content(Some("Hello World"));
}

pub trait Init {}
