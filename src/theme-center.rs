use crate::css_model::CssRenderList;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Mutex, MutexGuard, PoisonError};
pub trait TrThemesCenter {
    fn new<P: AsRef<Path>>(css_folder: P, default_theme_name: &str, regex: Option<&str>) -> Self;
    fn default_init() -> Self;
    fn get_default_theme(&self) -> String;
    fn set_default_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>>;
    fn change_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>>;
    fn get_render(&self, widget: &str) -> String;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ThemesCenter {
    default_theme: Mutex<String>,
    css_render_list: CssRenderList,
    render_list: HashMap<String, HashMap<String, String>>,
    theme: Vec<String>,
    render_now: HashMap<String, Mutex<String>>,
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
        }
        let render_now = {
            let mut now = HashMap::new();
            let first_render: &HashMap<String, String> = &render_list[&default_theme];
            for (k, v) in first_render.iter() {
                now.insert(k.to_string(), Mutex::new(v.to_string()));
            }
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
            mt.clear();
            mt.push_str(theme);
        })
    }
    fn change_theme(&self, theme: &str) -> Result<(), PoisonError<MutexGuard<String>>> {
        let mut res = Ok(());
        for (key, _val) in self.render_now.iter() {
            res = self.render_now[key].lock().map(|mut widget| {
                widget.clear();
                widget.push_str(&self.render_list[theme][key])
            });
            if res.is_err() {
                return res;
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
