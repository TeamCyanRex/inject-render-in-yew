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
include!(r"..\theme-center-metadata.rs"); //include! that commented above
create_theme_center!(MATATABLE_JSON); //create the static ref of theme center create by deserializing MATATABLE_JSON
#[test]
fn create_theme_center_test() {
    assert_eq!(get_default_theme(), "dark"); //get default theme in the theme center create by create_theme_center marco
    assert!(set_default_theme("lab").is_ok()); //set default theme in the theme center create by create_theme_center marco
    assert_eq!(get_default_theme(), "lab"); //get default theme after changed to lab
    assert_eq!(get_render("color"), "dark-color"); //get css class name of color widgit now in the theme center create by create_theme_center marco
    assert!(change_theme("river").is_ok()); //change theme now in the theme center create by create_theme_center marco
    assert_eq!(get_render("color"), "river-color"); //get css class name of color widgit now after theme changed to river
}
