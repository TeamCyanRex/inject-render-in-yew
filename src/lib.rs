#![doc = "inject_render_in_yew"]
//! # pub mod inject_render_in_yew
//!
//! # What is it?
//!
//! this mod is root of the inject_render_in_yew,
//! let you use global static struct create by marco and global function to manage and use css class neatly and esaily in yew
//!
//! # What can it do?
//!
//! let you use global static struct create by marco and global function to manage and use css class neatly and esaily in yew(the demo see in examples\demo-inject-render-in-yew).
//! let you can create your own custom theme center metadata creator(the demo see in example\render-metatable-creator).
//!
//! # Warning
//! the css files in the chosen directory must have form:
//! filename:{theme name}.css
//! css content:
//! only class!
//! .{theme name}-{widgit name}{...(what you want)...}
//! and every css file must have all widgits you have defined in the default theme,otherwise it can not work properly
//!
//! # import its public functional parts
//! ```
//! use inject_render_in_yew;
//! use inject_render_in_yew::css_model;
//! use inject_render_in_yew::theme_center;
//! use inject_render_in_yew::create_theme_center_json;
//! use inject_render_in_yew::prelude;
//! use inject_render_in_yew::create_theme_center;
//! ```

#[path = "./create-theme-center-json.rs"]
pub mod create_theme_center_json; //export mod crete_theme_center_json
#[path = "./create-theme-center-json-test.rs"]
mod create_theme_center_json_test;
#[path = "css-model.rs"]
pub mod css_model; //export mod css_model
#[path = "./css-model-test.rs"]
mod css_model_test;
#[path = "theme-center.rs"]
pub mod theme_center; //export mod theme_center
#[path = "./theme-center-test.rs"]
mod theme_center_test;

#[path = "./prelude.rs"]
pub mod prelude; //export mod prelude

extern crate lazy_static;

/// this macro is used to create the theme center which contain the metadata and methods for use of css files used by yew.
///
/// has two override
///
/// create_theme_center!($json:expr)
/// $json is the static str(json type) of the theme center
///
/// create_theme_center!($json:expr,$my_theme_center_type:ty)
/// $json is static str(json type) of the theme center
/// $my_theme_center_type is the custom theme center implied TrThemesCenter
///
/// # Example
///
/// ```
/// //It is a marco related,so doc test doesn't run code under this line 
/// //use inject_render_in_yew::create_theme_center;
/// //include!("../theme-center-metadata.rs");
/// //create_theme_center!(MATATABLE_JSON);or create_theme_center!(MATATABLE_JSON,CustomThemesCenter);
/// ```
#[macro_export]
macro_rules! create_theme_center {
        ($json:expr) => {
            use lazy_static::lazy_static;
            use serde_json::from_str;
            use $crate::theme_center::{ThemesCenter, TrThemesCenter};
            use std::sync::{Mutex, MutexGuard, PoisonError};
            lazy_static!{
                static ref THEME_CENTER:ThemesCenter=from_str($json)
                .map_or_else(|_e|panic!("Initialization themes render failed!You might use the render-metatable-creator to create meta json string!"), |obj|obj);
            }
            pub fn get_render(widget: &str) -> String {
                THEME_CENTER.get_render(widget)
            }
            pub fn change_theme(theme: &str)-> Result<(), PoisonError<MutexGuard<String>>> {
                THEME_CENTER.change_theme(theme)
            }
            pub fn get_default_theme() -> String {
                THEME_CENTER.get_default_theme()
            }
            pub fn set_default_theme(theme: &str)-> Result<(), PoisonError<MutexGuard<String>>> {
                THEME_CENTER.set_default_theme(theme)
            }
        };
        ($json:expr,$my_theme_center_type:ty) => {
            use lazy_static::lazy_static;
            use serde_json::from_str;
            use $crate::theme_center::{ThemesCenter, TrThemesCenter};
            use std::sync::{Mutex, MutexGuard, PoisonError};
            lazy_static!{
                static ref THEME_CENTER:$my_theme_center_type=from_str($json)
                .map_or_else(|_e|panic!("Initialization themes render failed!You might use the render-metatable-creator to create meta json string!"), |obj|obj);
            }
            pub fn get_render(widget: &str) -> String {
                THEME_CENTER.get_render(widget)
            }
            pub fn change_theme(theme: &str)-> Result<(), PoisonError<MutexGuard<String>>> {
                THEME_CENTER.change_theme(theme)
            }
            pub fn get_default_theme() -> String {
                THEME_CENTER.get_default_theme()
            }
            pub fn set_default_theme(theme: &str)-> Result<(), PoisonError<MutexGuard<String>>> {
                THEME_CENTER.set_default_theme(theme)
            }
        }
    }
