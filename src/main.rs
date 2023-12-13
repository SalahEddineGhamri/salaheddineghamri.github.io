use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{Navbar, Footer};
use crate::routers::{Route, switch};

mod components;
mod routers;
mod pages;

#[function_component]
fn App() -> Html {
    html! {
    <>
        <BrowserRouter>
            <Navbar/>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        <Footer/>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
