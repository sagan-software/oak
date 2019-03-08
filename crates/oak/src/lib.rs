pub use oak_app as app;
pub use oak_dom_node::html;

pub mod dom {
    pub use oak_dom_browser::*;
    pub use oak_dom_diff::*;
    pub use oak_dom_node::*;
}

pub mod prelude {
    pub use crate::{
        app::{self, *},
        dom::*,
        html::*,
    };
    pub use wasm_bindgen::prelude::*;
}
