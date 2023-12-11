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
            <div class="footer">
                <h2>{"Footer"}</h2>
            </div>
        }
    }
}
