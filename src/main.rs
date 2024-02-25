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
    <div class="flex flex-col min-h-screen">
        <HashRouter>
            <Navbar/>
            <div class="flex flex-col min-h-full">
            <div class="flex-1">
                <Switch<Route> render={switch} />
            </div>
            </div>
        </HashRouter>

        <div class="mt-auto">
            <Footer/>
        </div>
    </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
