#[allow(unused_imports)]
use crate::create_theme_center_json::{
    add_rs_suffix, add_slash_before_every_quota, create_render_metatable_json,
    create_render_metatable_path, output_to_render_metatable, process_json_to_rs_static_metadata,
    theme_center_json_string,write_into_metatable
};
#[allow(unused_imports)]
use std::path::PathBuf;

#[test]
fn write_into_metatable_test(){
    assert!(write_into_metatable(r"others\already", "some text").is_err());
    let res=write_into_metatable(r"others\new", "some text");
    assert!(res.is_ok());
    assert_eq!(String::from_utf8(std::fs::read(r"others\new").unwrap()).unwrap(),"some text");
    let _=std::fs::remove_file(r"others\new");

}
#[test]
fn add_rs_suffix_test() {
    assert_eq!("theme-center-metadata.rs", add_rs_suffix(""));
    assert_eq!(".rs", add_rs_suffix("."));
    assert_eq!("x.rs", add_rs_suffix("x"));
    assert_eq!(".rs", add_rs_suffix(".r"));
    assert_eq!("x.rs", add_rs_suffix("x."));
    assert_eq!(".rs", add_rs_suffix(".rs"));
    assert_eq!("x.rs", add_rs_suffix("x.r"));
    assert_eq!("xx.rs", add_rs_suffix("xx."));
    assert_eq!("x.rs", add_rs_suffix("x.rs"));
    assert_eq!("x.x.rs", add_rs_suffix("x.x."));
    assert_ne!("x.x.r.rs", add_rs_suffix("x.x.r"));
    assert_eq!("x.x.rs", add_rs_suffix("x.x"));
}

#[test]
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
    .join("");
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
    .join("");
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
    .join("");
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
    .join("");
    let res1 = (0..150)
        .map(|_| {
            create_render_metatable_json(Some("dist"), Some("lab"), None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&dist_lab)
        })
        .any(|b| b);
    let res2 = (0..150)
        .map(|_| {
            create_render_metatable_json(Some("dist"), None, None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&dist_dark)
        })
        .any(|b| b);
    let res3 = (0..150)
        .map(|_| {
            create_render_metatable_json(None, Some("lab"), None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&css_lab)
        })
        .any(|b| b);
    let res4 = (0..150)
        .map(|_| {
            create_render_metatable_json(None, None, None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&css_dark)
        })
        .any(|b| b);
    assert!(res1 && res2 && res3 && res4);
}
#[test]
fn create_render_metatable_path_test() {
    assert_eq!(
        create_render_metatable_path(Some("")),
        PathBuf::from("theme-center-metadata.rs")
    );
    assert_eq!(
        create_render_metatable_path(Some("xx")),
        PathBuf::from("xx.rs")
    );
    assert_eq!(
        create_render_metatable_path(None),
        PathBuf::from("theme-center-metadata.rs")
    );
}
#[test]
fn output_to_render_metatable_test() {
    output_to_render_metatable(None, &create_render_metatable_json(None, None, None));
}

#[test]
fn add_slash_to_every_quota_test() {
    let src = "{\"}";
    let res = add_slash_before_every_quota(src);
    assert_eq!("{\\\"}", &res);
}
#[test]
fn process_json_to_rs_static_metadata_test() {
    let tester = "static MATATABLE_JSON:&'static str=\"{\\\"}\";";
    let after = process_json_to_rs_static_metadata("{\"}");
    assert_eq!(tester, after);
}
#[test]
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
    .join("");
    let res = (0..200)
        .map(|_| {
            theme_center_json_string("css", "lab", None)
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .eq(&tester)
        })
        .any(|b| b);
    assert!(res);
}
