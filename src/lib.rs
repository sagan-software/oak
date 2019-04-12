pub mod prelude {
    pub use crate::{
        app::{App, AppResult},
        html::{attributes::*, events::*, *},
        vdom::*,
    };
    pub use wasm_bindgen::prelude::*;
}

#[macro_use]
mod vdom;

pub mod app;
pub mod node;

mod batch;
mod cmd;
mod diff;

mod html;

mod idle;
mod sub;
