#[allow(unused_imports)]
use crate::theme_center::{ThemesCenter, TrThemesCenter};

#[test]
fn themes_center_default_test() {
    let tester = ThemesCenter::default_init();
    assert_eq!(tester.get_default_theme(), "dark");
    assert!(tester.set_default_theme("lab").is_ok());
    assert_eq!(tester.get_default_theme(), "lab");
    assert_eq!(tester.get_render("color"), "dark-color");
    assert!(tester.change_theme("lab").is_ok());
    assert_eq!(tester.get_render("color"), "lab-color");
}
#[test]
#[should_panic]
fn themes_center_get_none_render() {
    let tester = ThemesCenter::default_init();
    tester.get_render("404");
}
#[test]
#[should_panic]
fn themes_center_change_none_theme() {
    let tester = ThemesCenter::default_init();
    let _ = tester.change_theme("404");
}
#[test]
#[should_panic]
fn themes_center_new_none_path() {
    let _ = ThemesCenter::new(r"404", "dark", None);
}
#[test]
#[should_panic]
fn themes_center_new_none_theme() {
    let _ = ThemesCenter::new(r"dist", "404", None);
}
#[test]
fn themes_center_new_test() {
    let tester = ThemesCenter::new(r"dist", "dark", None);
    assert_eq!(tester.get_default_theme(), "dark");
    assert!(tester.set_default_theme("lab").is_ok());
    assert_eq!(tester.get_default_theme(), "lab");
    assert_eq!(tester.get_render("color"), "dark-color");
    assert!(tester.change_theme("lab").is_ok());
    assert_eq!(tester.get_render("color"), "lab-color");
}
