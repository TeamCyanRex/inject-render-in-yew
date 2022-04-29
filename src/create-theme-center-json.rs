use crate::theme_center::{ThemesCenter, TrThemesCenter};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub(crate) fn create_render_metatable_path(out_path_opt: Option<&str>) -> PathBuf {
    match out_path_opt {
        Some(out_path) => PathBuf::from(add_rs_suffix(out_path)),
        _ => PathBuf::from(r"theme-center-metadata.rs"),
    }
}
pub(crate) fn write_into_metatable<P: AsRef<Path>>(
    out_path: P,
    la_rs: &str,
) -> Result<(), io::Error> {
    fs::write(out_path, la_rs)
}
pub(crate) fn add_rs_suffix(src: &str) -> String {
    let len = src.len();
    if len < 3 {
        if &src[len - 1..len] == "." {
            src.to_string() + "rs"
        } else {
            src.to_string() + ".rs"
        }
    } else if &src[len - 5..len] != ".rs" {
        if &src[len - 1..len] == "." {
            src.to_string() + "rs"
        } else {
            src.to_string() + ".rs"
        }
    } else {
        src.to_string()
    }
}
pub fn process_json_to_rs_static_metadata<'a>(src:&'a str) ->&'a str{
    src
}
pub fn output_to_render_metatable(out_path_opt: Option<&str>, la_rs_in: &str) {
    let la_rs=process_json_to_rs_static_metadata(la_rs_in);
    let mut out_path = create_render_metatable_path(out_path_opt).to_path_buf();
    let suffix_name = "-alter";
    while let Err(_e) = write_into_metatable(&out_path, la_rs) {
        if let Some(Some(file_name)) = out_path.file_name().map(|os| os.to_str()) {
            out_path = PathBuf::from(
                (&file_name[0..file_name.len() - 3]).to_owned() + suffix_name + ".rs",
            );
        } else {
            panic!("can not change the output render metatable path which existed");
        }
    }
}
pub fn create_render_metatable_rs(
    css_folder_opt: Option<&str>,
    default_theme_opt: Option<&str>,
    regex: Option<&str>,
) -> String {
    match css_folder_opt {
        Some(css_folder) => match default_theme_opt {
            Some(default_theme) => theme_center_json_string(css_folder, default_theme, regex),
            _ => theme_center_json_string(css_folder, "dark", regex),
        },
        _ => match default_theme_opt {
            Some(default_theme) => theme_center_json_string("css", default_theme, regex),
            _ => theme_center_json_string("css", "dark", regex),
        },
    }
}
pub(crate) fn theme_center_json_string(
    css_folder: &str,
    default_theme: &str,
    regex: Option<&str>,
) -> String {
    match serde_json::to_string_pretty(&ThemesCenter::new(css_folder, default_theme, regex)) {
        Ok(rs) => rs,
        Err(e) => e.to_string() + "!!!error occur,failed to spawn create metadata!",
    }
}
