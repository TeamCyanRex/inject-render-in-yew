#![doc = "inject_render_in_yew/src/theme_center.rs"]
//! # pub mod theme_center
//!
//! # What is it?
//!
//! this mod is used to create the theme center which contain the metadata and methods for use of css files used by yew.
//! and provides a trait to create your own theme center!
//!
//! # What can it do?
//!
//! you can use the trait TrThemesCenter to create your own theme center which can be initialized by using the create_theme_center! macro
//! and we don't recommend you to use this directly.
//!
//! # import its public functional parts
//! ```
//! use inject_render_in_yew::theme_center;
//! use inject_render_in_yew::theme_center::TrThemesCenter;
//! use inject_render_in_yew::theme_center::ThemesCenter;
//! ```

use crate::css_model::CssRenderList;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// This trait is used to create your own theme center
///
/// # Example
///
/// ```
/// impl TrThemesCenter for CustomThemesCenter {
///    fn new<P: AsRef<Path>>(css_folder: P, default_theme_name: &str, regex: Option<&str>) -> Self{...}
///    fn default_init() -> Self{...}
///    fn get_default_theme(&self) -> String{...}
///    fn set_default_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>>{...}
///    fn change_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>>{...}
///    fn get_render(&self, widget: &str) -> String{...}
/// }
/// ```
pub trait TrThemesCenter {
    /// create a new theme center with parameters
    fn new<P: AsRef<Path>>(css_folder: P, default_theme_name: &str, regex: Option<&str>) -> Self;
    /// Create a new theme center with default parameters
    fn default_init() -> Self;
    /// get the default theme name
    fn get_default_theme(&self) -> String;
    /// set the default theme with a new theme
    fn set_default_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>>;
    /// change the theme now yew used
    fn change_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>>;
    /// get the css class of the widget under now theme
    fn get_render(&self, widget: &str) -> String;
}

/// This struct is the default theme center<br/>
/// contain the metadata and methods for use of css files used by yew
///
/// # Example
///
/// ```
///
/// let themes_center=ThemesCenter::new(r"dist", "dark", None); or ThemesCenter::default_init();
/// themes_center.get_default_theme();
/// themes_center.set_default_theme("light");
/// themes_center.change_theme("sky");
/// let text_class=themes_center.get_render("text");
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ThemesCenter {
    /// the theme for start to load
    default_theme: Mutex<String>,
    /// the metadata create of all vaild css files
    css_render_list: CssRenderList,
    /// the hashmap of theme=>{widgit=>css_class}
    render_list: HashMap<String, HashMap<String, String>>,
    /// all themes
    theme: Vec<String>,
    /// the css classes of widgits now
    render_now: HashMap<String, Mutex<String>>,
    /// the number of themes
    theme_count: usize,
}
impl TrThemesCenter for ThemesCenter {
    fn new<P: AsRef<Path>>(
        css_folder: P,
        default_theme_name: &str,
        regex: Option<&str>,
    ) -> ThemesCenter {
        let default_theme = String::from(default_theme_name);
        let css_render_list = CssRenderList::new(css_folder, regex)
        .map_or_else(||panic!("can not create css render list correctly,mostly you should check the css path parameter"), |crl|crl);
        let render_list = css_render_list.renderlist.clone();
        let theme = css_render_list.themelist.clone();
        match theme.iter().find(|&now| now == &default_theme) {
            Some(_) => {}
            None => panic!("default theme not found in the available theme list"),
        } //confirm the default theme is in the themes
        let render_now = {
            let mut now = HashMap::new();
            let first_render: &HashMap<String, String> = &render_list[&default_theme]; //use default theme as start theme
            for (k, v) in first_render.iter() {
                now.insert(k.to_string(), Mutex::new(v.to_string()));
            } //wrap the css class strings by mutex,so we can change them safely in the future
            now
        };
        let theme_count = theme.len();
        Self {
            default_theme: Mutex::new(default_theme),
            css_render_list: css_render_list,
            render_list: render_list,
            theme: theme,
            render_now: render_now,
            theme_count: theme_count,
        }
    }
    fn default_init() -> Self {
        Self::new("dist", "dark", None)
    }
    fn get_default_theme(&self) -> String {
        match self.default_theme.lock().map(|mt| mt.to_string()) {
            Ok(res) => res,
            _ => panic!("from get_default_theme:get default theme name from poisoned mutex"),
        }
    }
    fn set_default_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>> {
        self.default_theme.lock().map(|mut mt| {
            mt.clear(); //empty string
            mt.push_str(theme); //change string to set theme
        })
    }
    fn change_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>> {
        let mut res = Ok(()); //no poisonerror now
        for (key, _val) in self.render_now.iter() {
            res = self.render_now[key].lock().map(|mut widget| {
                widget.clear();
                widget.push_str(&self.render_list[theme][key])
            }); //change widgit's class to {new theme}-{widgit name}
            if res.is_err() {
                return res; //poison now,return error!
            }
        }
        res
    }
    fn get_render(&self, widget: &str) -> String {
        match self.render_now[widget]
            .lock()
            .map(|mutex_res| mutex_res.to_string())
        {
            Ok(res) => res,
            _ => panic!("from get_render:get themed widget name from poisoned mutex"),
        }
    }
}
