use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
//use std::ffi::OsStr;
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
pub fn is_css_file<P: AsRef<Path>>(path: P) -> bool {
    let suffix = path.as_ref().extension();
    let suffix_str = suffix.map(|s| s.to_str());
    suffix_str == Some(Some("css"))
}
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
pub(crate) fn extract_css_basic_class<P: AsRef<Path>>(
    path: P,
    regex: Option<&str>,
) -> Result<Vec<String>, CssModelError> {
    let classes_res = fs::read_to_string(&path);
    if let Ok(classes) = classes_res {
        let pattern_res = Regex::new(regex.unwrap_or_else(|| r"\..+\-([0-9[:alpha:]\-_]+)\s*\{"));
        match pattern_res {
            Ok(pattern) => {
                let mut cursor = io::Cursor::new(classes);
                let mut buffer = String::new();
                let mut res = vec![];
                while let Ok(num) = cursor.read_line(&mut buffer) {
                    if num == 0 {
                        break;
                    }
                    let class_name_opt = pattern.captures(&buffer);
                    if let Some(class_name) = class_name_opt {
                        res.push(class_name[1].to_string());
                    }
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
pub(crate) fn remove_hash_suffix(tar: &str) -> String {
    if let Some(index) = tar.find('-') {
        tar[0..index].to_owned()
    } else {
        tar.to_owned()
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CssRenderList {
    pub themelist: Vec<String>,
    pub widgitlist: Vec<String>,
    pub renderlist: HashMap<String, HashMap<String, String>>,
}
impl CssRenderList {
    pub(crate) fn new<P: AsRef<Path>>(path: P, regex: Option<&str>) -> Option<Self> {
        if let Ok(vec_csses) = walk_dir_for_css(path) {
            if let Some(first_css) = vec_csses.first() {
                let mut widgitlist = vec![];
                let mut themelist = vec![];
                let widgets: Vec<String> = match extract_css_basic_class(first_css.as_ref(), regex)
                {
                    Ok(w) => w,
                    _ => return Default::default(),
                };
                widgets.into_iter().for_each(|s| widgitlist.push(s));

                for css in vec_csses.iter() {
                    let os_str = css.file_name().map(|s| match PathBuf::from(s).file_stem() {
                        Some(s) => s.to_os_string(),
                        _ => return Default::default(),
                    });
                    let theme_opt = os_str.map(|s| match s.to_str() {
                        Some(s) => s.to_string(),
                        _ => return Default::default(),
                    });
                    if let Some(theme) = theme_opt {
                        themelist.push(remove_hash_suffix(&theme));
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
