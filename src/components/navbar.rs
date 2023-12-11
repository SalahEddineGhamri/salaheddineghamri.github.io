use yew::prelude::*;

pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Navbar
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="navbar">
                <a href="#">{"Link"}</a>
                <a href="#">{"Link"}</a>
                <a href="#">{"Link"}</a>
                <a href="#" class="right">{"Link"}</a>
            </div>
        }
    }
}
