pub mod prelude {
    pub use crate::{
        app::{App, AppResult},
        core::*,
        diff::*,
        html::{attributes::*, events::*, *},
        idle::Idle,
        vdom::*,
    };
    pub use wasm_bindgen::prelude::*;
}

#[macro_use]
mod vdom;

pub mod app;
mod core;
pub mod node;

pub mod diff;

mod html;

mod idle;

// #[cfg(feature = "use-time")]
// pub mod time;
