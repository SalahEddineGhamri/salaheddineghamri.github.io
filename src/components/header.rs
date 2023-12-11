use yew::prelude::*;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Header
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="text-3xl font-bold underline">
                <h1>{"My website"}</h1>
                <p>{"a website created by me."}</p>
            </div>
        }
    }
}
