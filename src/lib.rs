#[path = "./create-theme-center-json.rs"]
pub mod create_theme_center_json;
#[path = "./create-theme-center-json-test.rs"]
mod create_theme_center_json_test;
#[path = "css-model.rs"]
pub mod css_model;
#[path = "./css-model-test.rs"]
mod css_model_test;
#[path = "theme-center.rs"]
pub mod theme_center;
#[path = "./theme-center-test.rs"]
mod theme_center_test;

#[path = "./prelude.rs"]
pub mod prelude;

extern crate lazy_static;

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
