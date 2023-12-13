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
            <div class="bottom-0 right-0 left-0 pb-8 justify-center fixed text-xs text-gray-600 text-center font-inter">
                <h2>{"all rights reserved 2024. website created by Salah Eddine Ghamri 🚀."}
                    <br/>
                    {"Powred by rust, yew, trunk and neovim."}</h2>
            </div>
        }
    }
}
