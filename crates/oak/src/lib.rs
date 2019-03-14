pub use oak_browser as browser;
pub use oak_core as core;
pub use oak_diff as diff;
pub use oak_html as html;
pub use oak_time as time;
pub use oak_vdom as vdom;

pub mod prelude {
    pub use crate::{
        browser::{App, AppResult},
        core::{futures::Future, js_sys::*, *},
        html::{attributes::*, events::*, *},
        time,
        vdom::*,
    };
    pub use wasm_bindgen::prelude::*;
}
