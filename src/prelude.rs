#![doc = "inject_render_in_yew/src/prelude.rs"]
//! # pub mod preclude
//!
//! # What is it?
//!
//! this mod is used to export the all api we recommend you to use directly
//!
//! # What can it do?
//!
//! mostly it is used to create your own custom theme center metadata creator
//!
//! # import its public functional parts
//! ```
//! use inject_render_in_yew::prelude;
//! use inject_render_in_yew::prelude::create_render_metatable_json;
//! use inject_render_in_yew::prelude::output_to_render_metatable;
//! ```

pub use crate::create_theme_center_json::{
    create_render_metatable_json, output_to_render_metatable,
};
