use crate::components::{Footer, Navbar};
use crate::routers::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routers;

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
