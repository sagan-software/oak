use oak_dom_node::Node as VirtualNode;
use wasm_bindgen::JsValue;
use web_sys::Node as BrowserNode;

pub struct App<S> {
    state: S,
    view: Box<dyn Fn(&S) -> VirtualNode>,
    root: BrowserNode,
    tree: VirtualNode,
}

pub trait Handler<T> {
    fn handle(&mut self, msg: &T);
}

pub fn with_state<S>(state: S) -> WithState<S> {
    WithState { state }
}

pub struct WithState<S> {
    state: S,
}

impl<S> WithState<S> {
    pub fn with_view<F>(self, func: F) -> WithView<S>
    where
        F: Fn(&S) -> VirtualNode + 'static,
    {
        WithView {
            state: self.state,
            view: Box::new(func),
        }
    }
}

pub fn stateless<V>(view: V) -> Stateless
where
    V: Into<VirtualNode>,
{
    Stateless { view: view.into() }
}

pub struct Stateless {
    view: VirtualNode,
}

impl Stateless {
    pub fn mount(self, selector: &str) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root = document.query_selector(selector)?.unwrap();
        let node = oak_dom_browser::create_dom_node(&self.view);
        root.set_inner_html("");
        root.append_child(&node.node)?;
        Ok(())
    }
}

pub struct WithView<S> {
    state: S,
    view: Box<dyn Fn(&S) -> VirtualNode>,
}

impl<S> WithView<S> {
    pub fn mount(self, selector: &str) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let root = document.query_selector(selector)?.unwrap();
        let tree = (self.view)(&self.state);
        let node = oak_dom_browser::create_dom_node(&tree);
        root.set_inner_html("");
        root.append_child(&node.node)?;
        let app = App {
            state: self.state,
            view: self.view,
            root: root.into(),
            tree,
        };
        Ok(())
    }
}
