use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="right-0 left-0 pb-8 justify-center text-xs text-gray-600 text-center font-inter">
                <h2>{"All rights reserved © 2023. website created by Salah Eddine Ghamri 🚀."}
                    <br/>
                    {"Powred by Rust🦀, Yew, Trunk and Neovim."}</h2>
            </div>
        }
    }
}
