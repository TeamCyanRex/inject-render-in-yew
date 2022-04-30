#![doc = "inject_render_in_yew/src/css-model.rs"]
//! # pub mod css_model
//!
//! # What is it?
//!
//! this mod is used to create a CssRenderList struct which contain the metadata of css files used by yew.
//!
//! # What can it do?
//!
//! It is used by mod theme_center,we don't recommend you to use this directly.
//!
//! # import its public functional parts
//! ```
//! use inject_render_in_yew::css_model;
//! use inject_render_in_yew::css_model::CssModelError;
//! use inject_render_in_yew::css_model::CssRenderList;
//! use inject_render_in_yew::css_model::is_css_file;
//! use inject_render_in_yew::css_model::walk_dir_for_css;
//! ```

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;
#[derive(Debug, PartialEq)]
pub enum CssModelError {
    ReadFileToStringError(String),
    CreateRegexError(String),
}
impl Display for CssModelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CssModelError::CreateRegexError(s) => write!(fmt, "can not create regex:{}!", s),
            CssModelError::ReadFileToStringError(s) => {
                write!(fmt, "can not read file to string,file path:{}!", s)
            }
        }
    }
}
impl Error for CssModelError {}

/// assert whether a file is css file or not
///
/// # Parameter
///
/// path is the path to the file
///
/// # Example
///
/// ```
/// assert_eq!(true, is_css_file(r"css\dark.css"));
/// assert_eq!(false, is_css_file(r"css\no.cs"));
/// ```
pub fn is_css_file<P: AsRef<Path>>(path: P) -> bool {
    let suffix = path.as_ref().extension();
    let suffix_str = suffix.map(|s| s.to_str());
    suffix_str == Some(Some("css"))
}
/// get all css file path from a folder
///
/// # Parameter
///
/// folder is the path to the folder contain css files
///
/// # Example
///
/// ```
/// let vec_pathbuf=walk_dir_for_css(r"css");
/// ```
pub fn walk_dir_for_css<P: AsRef<Path>>(folder: P) -> io::Result<Vec<Box<PathBuf>>> {
    let path = folder.as_ref();
    let mut res_vec = vec![];
    for entry in fs::read_dir(path)? {
        let css_file = entry?.path();
        let css_file_out = css_file.clone();
        if is_css_file(css_file) {
            res_vec.push(Box::new(css_file_out));
        }
    }
    Ok(res_vec)
}
/// extract all widgits used in a css file properly with regex
///
/// # Parameter
///
/// path is the path to the css file<br/>
/// regex is the regex to search for widgit name
///
/// # Example
///
/// ```
/// let widgets=extract_css_basic_class(r"dark.css");
/// ```
pub(crate) fn extract_css_basic_class<P: AsRef<Path>>(
    path: P,
    regex: Option<&str>,
) -> Result<Vec<String>, CssModelError> {
    let classes_res = fs::read_to_string(&path); //read css file's content to string
    if let Ok(classes) = classes_res {
        let pattern_res = Regex::new(regex.unwrap_or_else(|| r"\..+\-([0-9[:alpha:]\-_]+)\s*\{")); //.{theme name}-(widget){some space}{
        match pattern_res {
            Ok(pattern) => {
                let mut cursor = io::Cursor::new(classes); //to read string line to line
                let mut buffer = String::new();
                let mut res = vec![];
                while let Ok(num) = cursor.read_line(&mut buffer) {
                    if num == 0 {
                        break;
                    } //to the end
                    let class_name_opt = pattern.captures(&buffer);
                    if let Some(class_name) = class_name_opt {
                        res.push(class_name[1].to_string());
                    } //read widgit name and store in result vector
                    buffer.clear()
                }
                Ok(res)
            }
            Err(_) => Err(CssModelError::CreateRegexError(
                regex
                    .unwrap_or_else(|| r"\..+\-([0-9[:alpha:]\-_]+)\s*\{")
                    .to_string(),
            )),
        }
    } else {
        Err(CssModelError::ReadFileToStringError(
            path.as_ref()
                .to_str()
                .map_or("path object broken".to_owned(), |s| s.to_string()),
        ))
    }
}

/// create tree from main trunk strings and sub trunk strings<br/>
/// {main=>{sub=>mian-sub}}
///
/// # Parameter
///
/// main_key is main trunk strings<br/>
/// sub_key is sub trunk strings
///
/// # Example
///
/// ```
/// let renderlist=cross_vec_str(
///            &vec!["dark".to_string(), "light".to_string()],
///            &vec!["color".to_string(), "size".to_string()]
///        );
/// ```
pub(crate) fn cross_vec_str(
    main_key: &Vec<String>,
    sub_key: &Vec<String>,
) -> HashMap<String, HashMap<String, String>> {
    let mut res = HashMap::new();
    for main in main_key.iter() {
        let mut subs = HashMap::new();
        for sub in sub_key.iter() {
            subs.insert(sub.clone(), main.to_owned() + "-" + sub);
        }
        res.insert(main.clone(), subs);
    }
    res
}
/// remove the hash like suffix from a name string<br>
/// hash like suffix="-{some_string}""
/// # Parameter
///
/// tar is the target string with hash like suffix
///
/// # Example
///
/// ```
///  assert_eq!("i", remove_hash_suffix("i-like"));
/// ```
pub(crate) fn remove_hash_suffix(tar: &str) -> String {
    if let Some(index) = tar.find('-') {
        //firet "-" locate index
        tar[0..index].to_owned()
    } else {
        tar.to_owned()
    }
}
/// This struct which contain the metadata of css files used by yew
///
/// # Example
///
/// ```
/// let themelist = vec![
/// "dark".to_string(),
/// "desert".to_string(),
/// "lab".to_string(),
/// "light".to_string(),
/// "river".to_string(),
/// ];
/// let widgitlist = vec!["color".to_string()];
/// let renderlist = cross_vec_str(&themelist, &widgitlist);
/// let css_render_list= CssRenderList {
/// themelist,
/// widgitlist,
/// renderlist,
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CssRenderList {
    /// all themes available
    pub themelist: Vec<String>,
    /// all widgits available
    pub widgitlist: Vec<String>,
    /// rendrlist:{<theme>=>{<widgit>=><theme>-<widgit>}}
    pub renderlist: HashMap<String, HashMap<String, String>>,
}
impl CssRenderList {
    pub(crate) fn new<P: AsRef<Path>>(path: P, regex: Option<&str>) -> Option<Self> {
        if let Ok(vec_csses) = walk_dir_for_css(path) {
            //get all vaild css files path from the folder path param point to
            if let Some(first_css) = vec_csses.first() {
                let mut widgitlist = vec![];
                let mut themelist = vec![];
                let widgets: Vec<String> = match extract_css_basic_class(first_css.as_ref(), regex)
                {
                    Ok(w) => w,
                    _ => return Default::default(),
                }; //from first css file extract all widgets available
                widgets.into_iter().for_each(|s| widgitlist.push(s));

                for css in vec_csses.iter() {
                    let os_str = css.file_name().map(|s| match PathBuf::from(s).file_stem() {
                        Some(s) => s.to_os_string(),
                        _ => return Default::default(),
                    }); //get css file name as OsString
                    let theme_opt = os_str.map(|s| match s.to_str() {
                        Some(s) => s.to_string(),
                        _ => return Default::default(),
                    }); //change OsString to String
                    if let Some(theme) = theme_opt {
                        themelist.push(remove_hash_suffix(&theme)); //extract theme name from css file name string
                    } else {
                        return Default::default();
                    }
                }
                let renderlist = cross_vec_str(&themelist, &widgitlist);
                Some(Self {
                    themelist: themelist,
                    widgitlist: widgitlist,
                    renderlist: renderlist,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
