use yew::prelude::*;
use crate::components::{Footer, Navbar};
use crate::routers::{switch, Route};

#[function_component]
pub fn App() -> Html {
    html! {
        <>
        <div class="flex flex-col min-h-screen">
            {
                // Only render router in WASM
                if cfg!(target_arch = "wasm32") {
                    use yew_router::prelude::*;
                    html! {
                        <HashRouter>
                            <Navbar/>
                            <div class="flex flex-col min-h-full">
                                <div class="flex-1">
                                    <Switch<Route> render={switch} />
                                </div>
                            </div>
                        </HashRouter>
                    }
                } else {
                    // SSR: fallback UI
                    html! {
                        <div>
                            <Navbar/>
                            <div class="flex flex-col min-h-full">
                                <div class="flex-1">{ "Server-side placeholder" }</div>
                            </div>
                        </div>
                    }
                }
            }

            <div class="mt-auto">
                <Footer/>
            </div>
        </div>
        </>
    }
}
