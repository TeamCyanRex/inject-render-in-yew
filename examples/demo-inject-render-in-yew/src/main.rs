use yew::prelude::*;
mod buttons;
use buttons::ButtonT;
use inject_render_in_yew::create_theme_center;
include!(r"../conf/theme-center-metadata.rs");
create_theme_center!(MATATABLE_JSON);

#[function_component(App)]
fn hello_world() -> Html {
    let _ = change_theme("lab");
    html! {
        <>
        <ButtonT val=1></ButtonT>
        <p class={classes!(get_render("color"))}>{String::from("lab")+"color"}</p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
