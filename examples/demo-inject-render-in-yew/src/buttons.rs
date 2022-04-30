use crate::get_render;
const MAX_PROPS: u32 = 200000;
const MIN_PROPS: u32 = 0;
use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub val: u32,

}
impl ButtonProps {
    fn new(val: u32) -> ButtonProps {
        ButtonProps { val: val }
    }
    fn add(&mut self) -> bool {
        if self.val < MAX_PROPS {
            self.val += 1;
            true
        } else {
            false
        }
    }
    fn minus(&mut self) -> bool {
        if self.val > MIN_PROPS {
            self.val -= 1;
            true
        } else {
            false
        }
    }
}
pub struct ButtonT {
    props: ButtonProps,
}
pub enum Message {
    Add,
    Minus,
}
impl Component for ButtonT {
    type Message = Message;
    type Properties = ButtonProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            props: ButtonProps::new(0),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Add => self.props.add(),
            Message::Minus => self.props.minus(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button class={classes!(get_render("color"))} id="big" onclick={link.callback(|_| Message::Add)}>{ "+1" }</button>
                <br/>
                <button onclick={link.callback(|_| Message::Minus)}>{ "-1" }</button>
                <br/>
                <p>{ self.props.val.to_string() }</p>
            </div>
        }
    }
}
