#[allow(unused_imports)]
use crate::create_theme_center_json::{
    add_rs_suffix, add_slash_before_every_quota, create_render_metatable_json,
    create_render_metatable_path, output_to_render_metatable, process_json_to_rs_static_metadata,
    theme_center_json_string, write_into_metatable,
};
#[allow(unused_imports)]
use std::path::PathBuf;

#[test]
fn write_into_metatable_test() {
    assert!(write_into_metatable(r"others\already", "some text").is_err()); //already is existed,so there is a io error
    let res = write_into_metatable(r"others\new", "some text"); //create file nameed new,write into "some text"
    assert!(res.is_ok());
    assert_eq!(
        String::from_utf8(std::fs::read(r"others\new").unwrap()).unwrap(),
        "some text"
    ); // read from new,and test whether it has "some text"
    let _ = std::fs::remove_file(r"others\new"); //remove file named new to prepare for next test
}
#[test]
fn add_rs_suffix_test() {
    assert_eq!("theme-center-metadata.rs", add_rs_suffix("")); //test when parameter is blank str
    assert_eq!(".rs", add_rs_suffix(".")); //test when parameter is 1 length as "."
    assert_eq!("x.rs", add_rs_suffix("x")); //test when parameter is 1 length and not "."
    assert_eq!(".rs", add_rs_suffix(".r")); //test when parameter is 2 length as ".r"
    assert_eq!("x.rs", add_rs_suffix("x.")); //test when parameter is 2 length as finish with "."
    assert_eq!(".rs", add_rs_suffix(".rs")); //test when parameter is 3 length as ".rs"
    assert_eq!("x.rs", add_rs_suffix("x.r")); //test when parameter is 3 length as finish with ".r"
    assert_eq!("xx.rs", add_rs_suffix("xx.")); //test when parameter is 3 length as finish with "."
    assert_eq!("x.rs", add_rs_suffix("x.rs")); //test when parameter is longer than 3 as finish with ".rs"
    assert_eq!("x.x.rs", add_rs_suffix("x.x.")); //test when parameter is longer than 3 as finish with "."
    assert_eq!("x.x.rs", add_rs_suffix("x.x.r")); //test when parameter is longer than 3 as finish with ".r"
    assert_eq!("x.x.rs", add_rs_suffix("x.x")); //test when parameter is longer than 3 generally
}

#[test]
#[ignore]
fn create_render_metatable_json_test() {
    let dist_lab = "{
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
            \"desert\": {
              \"color\": \"desert-color\"
            },
            \"lab\": {
              \"color\": \"lab-color\"
            },
            \"light\": {
              \"color\": \"light-color\"
            },
            \"dark\": {
              \"color\": \"dark-color\"
            },
            \"river\": {
              \"color\": \"river-color\"
            }
          }
        },
        \"render_list\": {
          \"desert\": {
            \"color\": \"desert-color\"
          },
          \"lab\": {
            \"color\": \"lab-color\"
          },
          \"light\": {
            \"color\": \"light-color\"
          },
          \"dark\": {
            \"color\": \"dark-color\"
          },
          \"river\": {
            \"color\": \"river-color\"
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
      }"
    .split_ascii_whitespace()
    .collect::<Vec<&str>>()
    .join(""); //theme center json with remove all ascii whitespaces in dist and default theme is lab
    let dist_dark = "{
        \"default_theme\": \"dark\",
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
            \"desert\": {
              \"color\": \"desert-color\"
            },
            \"river\": {
              \"color\": \"river-color\"
            },
            \"dark\": {
              \"color\": \"dark-color\"
            },
            \"light\": {
              \"color\": \"light-color\"
            },
            \"lab\": {
              \"color\": \"lab-color\"
            }
          }
        },
        \"render_list\": {
          \"desert\": {
            \"color\": \"desert-color\"
          },
          \"river\": {
            \"color\": \"river-color\"
          },
          \"dark\": {
            \"color\": \"dark-color\"
          },
          \"light\": {
            \"color\": \"light-color\"
          },
          \"lab\": {
            \"color\": \"lab-color\"
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
          \"color\": \"dark-color\"
        },
        \"theme_count\": 5
      }"
    .split_ascii_whitespace()
    .collect::<Vec<&str>>()
    .join(""); //theme center json with remove all ascii whitespaces in dist and default theme is dark
    let css_lab = "{
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
            \"desert\": {
              \"color\": \"desert-color\"
            },
            \"light\": {
              \"color\": \"light-color\"
            },
            \"dark\": {
              \"color\": \"dark-color\"
            },
            \"river\": {
              \"color\": \"river-color\"
            },
            \"lab\": {
              \"color\": \"lab-color\"
            }
          }
        },
        \"render_list\": {
          \"desert\": {
            \"color\": \"desert-color\"
          },
          \"light\": {
            \"color\": \"light-color\"
          },
          \"dark\": {
            \"color\": \"dark-color\"
          },
          \"river\": {
            \"color\": \"river-color\"
          },
          \"lab\": {
            \"color\": \"lab-color\"
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
      }"
    .split_ascii_whitespace()
    .collect::<Vec<&str>>()
    .join(""); //theme center json with remove all ascii whitespaces in css and default theme is lab
    let css_dark = "{
        \"default_theme\": \"dark\",
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
            \"desert\": {
              \"color\": \"desert-color\"
            },
            \"dark\": {
              \"color\": \"dark-color\"
            },
            \"light\": {
              \"color\": \"light-color\"
            },
            \"lab\": {
              \"color\": \"lab-color\"
            },
            \"river\": {
              \"color\": \"river-color\"
            }
          }
        },
        \"render_list\": {
          \"desert\": {
            \"color\": \"desert-color\"
          },
          \"dark\": {
            \"color\": \"dark-color\"
          },
          \"light\": {
            \"color\": \"light-color\"
          },
          \"lab\": {
            \"color\": \"lab-color\"
          },
          \"river\": {
            \"color\": \"river-color\"
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
          \"color\": \"dark-color\"
        },
        \"theme_count\": 5
      }"
    .split_ascii_whitespace()
    .collect::<Vec<&str>>()
    .join(""); //theme center json with remove all ascii whitespaces in css and default theme is dark
    let res1 = (0..500)
        .map(|_| {
            create_render_metatable_json(Some("dist"), Some("lab"), None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&dist_lab)
        })
        .any(|b| b); //run 500 times to ensure the sequence of hashMap type entrys is same as dist_lab where are related
    let res2 = (0..500)
        .map(|_| {
            create_render_metatable_json(Some("dist"), None, None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&dist_dark)
        })
        .any(|b| b); //run 500 times to ensure the sequence of hashMap type entrys is same as dist_dark where are related
    let res3 = (0..500)
        .map(|_| {
            create_render_metatable_json(None, Some("lab"), None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&css_lab)
        })
        .any(|b| b); //run 500 times to ensure the sequence of hashMap type entrys is same as css_lab where are related
    let res4 = (0..500)
        .map(|_| {
            create_render_metatable_json(None, None, None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&css_dark)
        })
        .any(|b| b); //run 500 times to ensure the sequence of hashMap type entrys is same as css_dark where are related
    assert!(res1 && res2 && res3 && res4); //assert all res are true
}
#[test]
fn create_render_metatable_path_test() {
    assert_eq!(
        create_render_metatable_path(Some("")),
        PathBuf::from("theme-center-metadata.rs")
    ); //create render metatable path with blank path
    assert_eq!(
        create_render_metatable_path(Some("xx")),
        PathBuf::from("xx.rs")
    ); //create_render_metatable_path with not blank path
    assert_eq!(
        create_render_metatable_path(None),
        PathBuf::from("theme-center-metadata.rs")
    ); //create render metatable path with default path
}
#[test]
fn output_to_render_metatable_test() {
    //create a render metatable with default parameters,it will create theme-center-metadata.rs or create theme-center-metadata(-alter)*.rs
    output_to_render_metatable(None, &create_render_metatable_json(None, None, None));
}

#[test]
fn add_slash_to_every_quota_test() {
    let src = "{\"}"; //in {} is "
    let res = add_slash_before_every_quota(src); //change " in {}  to \"
    assert_eq!("{\\\"}", &res);
}
#[test]
fn process_json_to_rs_static_metadata_test() {
    let tester = "static MATATABLE_JSON:&'static str=\"{\\\"}\";";
    let after = process_json_to_rs_static_metadata("{\"}"); //change {"} to static MATATABLE_JSON:&'static str="{\"}"
    assert_eq!(tester, after);
}
#[test]
#[ignore]
fn theme_center_json_string_test() {
    let tester = "{
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
      }"
    .split_ascii_whitespace()
    .collect::<Vec<&str>>()
    .join(""); //theme center json with remove all ascii whitespaces in css and default theme is lab
    let res = (0..500)
        .map(|_| {
            theme_center_json_string("css", "lab", None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&tester)
        })
        .any(|b| b); //run 500 times to ensure the sequence of hashMap type entrys is same as css_lab where are related
    assert!(res);
}
