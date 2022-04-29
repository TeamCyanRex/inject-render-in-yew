#[allow(unused_imports)]
use crate::css_model::{
    cross_vec_str, extract_css_basic_class, is_css_file, remove_hash_suffix, walk_dir_for_css,
    CssModelError, CssRenderList,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::path::PathBuf;

#[test]
fn css_render_list_test() {
    let themelist = vec![
        "dark".to_string(),
        "desert".to_string(),
        "lab".to_string(),
        "light".to_string(),
        "river".to_string(),
    ];
    let widgitlist = vec!["color".to_string()];
    let renderlist = cross_vec_str(&themelist, &widgitlist);
    let tester = CssRenderList {
        themelist,
        widgitlist,
        renderlist,
    };
    assert_eq!(tester, CssRenderList::new(r"css", None).unwrap());
    assert!(CssRenderList::new(r"404",None).is_none());
}

#[test]
fn walk_dir_for_css_test() {
    assert!(walk_dir_for_css("404").is_err());
    assert_eq!(5, walk_dir_for_css(r"css").map(|v| v.len()).unwrap());
}
#[test]
fn is_css_file_test() {
    assert_eq!(true, is_css_file(r"css\dark.css"));
    assert_eq!(false, is_css_file(r"css\no.cs"));
}
#[test]
fn extract_css_basic_class_test() {
    assert_eq!(
        Ok(vec!["color".to_string()]),
        extract_css_basic_class(r"css\dark.css", None)
    );
    assert_eq!(
        Err(CssModelError::CreateRegexError(r"\p".to_string())),
        extract_css_basic_class(r"css\dark.css", Some(r"\p"))
    );
    assert_eq!(
        Err(CssModelError::ReadFileToStringError("./404".to_string())),
        extract_css_basic_class(r"./404", None)
    );
}
#[test]
fn cross_vec_str_test() {
    let mut tester = HashMap::new();
    tester.insert("dark".to_string(), {
        let mut temp = HashMap::new();
        temp.insert("color".to_string(), "dark-color".to_string());
        temp.insert("size".to_string(), "dark-size".to_string());
        temp
    });
    tester.insert("light".to_string(), {
        let mut temp = HashMap::new();
        temp.insert("color".to_string(), "light-color".to_string());
        temp.insert("size".to_string(), "light-size".to_string());
        temp
    });
    assert_eq!(
        tester,
        cross_vec_str(
            &vec!["dark".to_string(), "light".to_string()],
            &vec!["color".to_string(), "size".to_string()]
        )
    );
    tester.insert("none".to_string(), {
        let mut temp = HashMap::new();
        temp.insert("color".to_string(), "light-color".to_string());
        temp.insert("size".to_string(), "light-size".to_string());
        temp
    });
    assert_ne!(
        tester,
        cross_vec_str(
            &vec!["dark".to_string(), "light".to_string()],
            &vec!["color".to_string(), "size".to_string()]
        )
    );
}
#[test]
fn remove_hash_suffix_test() {
    assert_eq!("ilike", remove_hash_suffix("ilike"));
    assert_eq!("i", remove_hash_suffix("i-like"));
    assert_eq!("ilike", remove_hash_suffix("ilike-"));
    assert_eq!("", remove_hash_suffix("-ilike"));
    assert_eq!("i", remove_hash_suffix("i-li-ke"));
    assert_ne!("ili", remove_hash_suffix("i-li-ke"))
}
