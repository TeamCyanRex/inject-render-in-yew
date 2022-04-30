use clap::{Arg, Command};
use inject_render_in_yew::prelude::{create_render_metatable_json, output_to_render_metatable};

fn main() {
    clap_launch();
}

fn clap_launch() {
    let matches = Command::new("Render-Metatable Creator")
        .version("0.1.1")
        .author("vicimeans vicimeans12358@outlook.com")
        .about("create json file of themecenter from css folder")
        .arg(
            Arg::new("css_folder")
                .long("folder")
                .short('f')
                .takes_value(true)
                .help("the path of the css folder"),
        )
        .arg(
            Arg::new("default_theme")
                .long("theme")
                .short('t')
                .takes_value(true)
                .help("the name of default theme"),
        )
        .arg(
            Arg::new("json_path")
                .long("json")
                .short('j')
                .takes_value(true)
                .help("path of result json file"),
        )
        .get_matches();
    output_to_render_metatable(
        matches.value_of("json_path"),
        &create_render_metatable_json(
            matches.value_of("css_folder"),
            matches.value_of("default_theme"),
            None,
        ),
    );
}