#[allow(unused_imports)]
use inject_render_in_yew::create_theme_center;
/*static MATATABLE_JSON: &'static str = "{
    \"default_theme\": \"lab\",
    \"css_render_list\": {
      \"themelist\": [
        \"dark\",
        \"desert\",
        \"lab\",
        \"light\",
        \"river\"
      ],
      \"widgitlist\": [
        \"color\"
      ],
      \"renderlist\": {
        \"lab\": {
          \"color\": \"lab-color\"
        },
        \"light\": {
          \"color\": \"light-color\"
        },
        \"river\": {
          \"color\": \"river-color\"
        },
        \"desert\": {
          \"color\": \"desert-color\"
        },
        \"dark\": {
          \"color\": \"dark-color\"
        }
      }
    },
    \"render_list\": {
      \"lab\": {
        \"color\": \"lab-color\"
      },
      \"light\": {
        \"color\": \"light-color\"
      },
      \"river\": {
        \"color\": \"river-color\"
      },
      \"desert\": {
        \"color\": \"desert-color\"
      },
      \"dark\": {
        \"color\": \"dark-color\"
      }
    },
    \"theme\": [
      \"dark\",
      \"desert\",
      \"lab\",
      \"light\",
      \"river\"
    ],
    \"render_now\": {
      \"color\": \"lab-color\"
    },
    \"theme_count\": 5
  }";
*/
include!(r"..\theme-center-metadata.rs");
create_theme_center!(MATATABLE_JSON);
#[test]
fn create_theme_center_test() {
    assert_eq!(get_default_theme(), "dark");
    assert!(set_default_theme("lab").is_ok());
    assert_eq!(get_default_theme(), "lab");
    assert_eq!(get_render("color"), "dark-color");
    assert!(change_theme("river").is_ok());
    assert_eq!(get_render("color"), "river-color");
}
