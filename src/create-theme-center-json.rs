#![doc = "inject_render_in_yew/src/create-theme-center-json.rs"]
//! # pub mod create_theme_center_json
//!
//! # What is it?
//!
//! this mod is used to create a final rust source code file with a static str(json type) of the theme center which contain the metadata and methods for use of css files used by yew.
//!
//! # What can it do?
//!
//! you can use this to create your own custom theme center metadata creator(the demo see in example\render-metatable-creator).
//!
//! # import its public functional parts
//! ```
//! use inject_render_in_yew::create_theme_center_json;
//! use inject_render_in_yew::create_theme_center_json::add_slash_before_every_quota;
//! use inject_render_in_yew::create_theme_center_json::create_render_metatable_json;
//! use inject_render_in_yew::create_theme_center_json::output_to_render_metatable;
//! use inject_render_in_yew::create_theme_center_json::process_json_to_rs_static_metadata;
//! ```

use crate::theme_center::{ThemesCenter, TrThemesCenter};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// create vaild theme center metadata path from a option str
///
/// # Parameter
///
/// out_path_opt is the target option str
///
/// # Example
///
/// ```
/// // It is private,so doc test doesn't run code under this line 
/// //use std::path::PathBuf;
/// //assert_eq!(
/// //  create_render_metatable_path(Some("")),
/// // PathBuf::from("theme-center-metadata.rs")
/// //);
/// //assert_eq!(
/// //  create_render_metatable_path(Some("xx")),
/// // PathBuf::from("xx.rs")
/// //);
/// //assert_eq!(
/// //  create_render_metatable_path(None),
/// //  PathBuf::from("theme-center-metadata.rs")
/// //);
/// ```
pub(crate) fn create_render_metatable_path(out_path_opt: Option<&str>) -> PathBuf {
    match out_path_opt {
        Some(out_path) => PathBuf::from(add_rs_suffix(out_path)), //change suffix to ".rs"
        _ => PathBuf::from(r"theme-center-metadata.rs"),
    }
}
/// write string into a file
///
/// # Parameter
///
/// out_path is target path<br/>
/// la_rs is the string will be writen
///
/// # Example
///
/// ```
/// //It is private,so doc test doesn't run code under this line 
/// //write_into_metatable(r"others\new", "some text");
/// ```
pub(crate) fn write_into_metatable<P: AsRef<Path>>(
    out_path: P,
    la_rs: &str,
) -> Result<(), io::Error> {
    if out_path.as_ref().exists() {
        Err(io::Error::from(io::ErrorKind::AlreadyExists))
    } else {
        fs::File::create(&out_path)?.write_all(la_rs.as_bytes())?;
        Ok(())
    }
}
/// change a string's suffix to ".rs"
///
/// # Parameter
///
/// src is the target string
///
/// # Example
///
/// ```
/// //It is private,so doc test doesn't run code under this line 
/// //assert_eq!("like.rs", add_rs_suffix("like"));
/// ```
pub(crate) fn add_rs_suffix(src: &str) -> String {
    let len = src.len();
    if len == 0 {
        "theme-center-metadata.rs".to_owned()
    } else if len == 1 {
        if &src[len - 1..len] == "." {
            //finish with "."
            src.to_string() + "rs"
        } else {
            src.to_string() + ".rs"
        }
    } else if len == 2 {
        if &src[len - 1..len] == "." {
            //finish with "."
            src.to_string() + "rs"
        } else if &src[len - 2..len] == ".r" {
            //finish with ".r"
            src.to_string() + "s"
        } else {
            src.to_string() + ".rs"
        }
    } else if &src[len - 3..len] != ".rs" {
        if &src[len - 1..len] == "." {
            //finish with "."
            src.to_string() + "rs"
        } else if &src[len - 2..len] == ".r" {
            //finish with ".r"
            src.to_string() + "s"
        } else {
            src.to_string() + ".rs"
        }
    } else {
        src.to_string()
    }
}
/// add a '\\' before every '"' in target string<br>
///
/// # Parameter
///
/// src is the target string
///
/// # Example
///
/// ```
///  use inject_render_in_yew::create_theme_center_json::add_slash_before_every_quota;
///  assert_eq!("\\\"i",add_slash_before_every_quota("\"i"));
/// ```
pub fn add_slash_before_every_quota(src: &str) -> String {
    let quota_splits: Vec<&str> = src.split('"').collect();
    quota_splits.join("\\\"")
}
/// wrap a target json string in the rust static str code line format<br>
///
/// # Parameter
///
/// src is the target json string
///
/// # Example
///
/// ```
/// use inject_render_in_yew::create_theme_center_json::process_json_to_rs_static_metadata;
/// let tester = "static MATATABLE_JSON:&'static str=\"{\\\"}\";";
/// let after = process_json_to_rs_static_metadata("{\"}");
/// assert_eq!(tester, after);
/// ```
pub fn process_json_to_rs_static_metadata(src: &str) -> String {
    let mut prefix = "static MATATABLE_JSON:&'static str=\"".to_owned();
    prefix.push_str(&add_slash_before_every_quota(src));
    prefix.push_str("\";");
    prefix
}
/// write theme center metadata to target file
///
///
/// # Parameter
///
/// out_path_opt is target file path<br/>
/// la_rs_in is theme center metadata
///
/// # Example
///
/// ```
/// use inject_render_in_yew::prelude::output_to_render_metatable;
/// use inject_render_in_yew::prelude::create_render_metatable_json;
/// output_to_render_metatable(None, &create_render_metatable_json(None, None, None));        
/// ```
pub fn output_to_render_metatable(out_path_opt: Option<&str>, la_rs_in: &str) {
    let la_rs = process_json_to_rs_static_metadata(la_rs_in);
    let mut out_path = create_render_metatable_path(out_path_opt).to_path_buf();
    let suffix_name = "-alter";
    while let Err(_e) = write_into_metatable(&out_path, &la_rs) {
        if let Some(Some(file_name)) = out_path.file_name().map(|os| os.to_str()) {
            out_path = PathBuf::from(
                (&file_name[0..file_name.len() - 3]).to_owned() + suffix_name + ".rs", //add a "-alter" between out path and ".rs"
            );
        } else {
            panic!("can not change the output render metatable path which existed");
        }
    }
}
/// create theme center metadata as string<br>
/// this function's parameters are all nullable
///
/// # Parameter
///
/// css_folder_opt is the used folder within css files<br/>
/// default_theme_opt is used default theme<br/>
/// regex is the regex extract widgit name from css file content lines
///
/// # Example
///
/// ```
/// use inject_render_in_yew::prelude::create_render_metatable_json;
/// create_render_metatable_json(Some("dist"), Some("lab"), None);
/// ```
pub fn create_render_metatable_json(
    css_folder_opt: Option<&str>,
    default_theme_opt: Option<&str>,
    regex: Option<&str>,
) -> String {
    match css_folder_opt {
        Some(css_folder) => match default_theme_opt {
            Some(default_theme) => theme_center_json_string(css_folder, default_theme, regex),
            _ => theme_center_json_string(css_folder, "dark", regex), //default theme is null
        },
        _ => match default_theme_opt {
            Some(default_theme) => theme_center_json_string("css", default_theme, regex), //css folder is null
            _ => theme_center_json_string("css", "dark", regex), //default theme and css folder are null
        },
    }
}
/// create theme center metadata as string
///
/// # Parameter
///
/// css_folder is the used folder within css files<br/>
/// default_theme is used default theme<br/>
/// regex is the regex extract widgit name from css file content lines
///
/// # Example
///
/// ```
/// // It is private,so doc test doesn't run code under this line 
/// //theme_center_json_string("css", "dark", None);
/// ```
pub(crate) fn theme_center_json_string(
    css_folder: &str,
    default_theme: &str,
    regex: Option<&str>,
) -> String {
    match serde_json::to_string_pretty(&ThemesCenter::new(css_folder, default_theme, regex)) //serialize themes center to json string
     {
       
        Ok(rs) => rs,
        Err(e) => e.to_string() + "!!!error occur,failed to spawn create metadata!",
    }
}
