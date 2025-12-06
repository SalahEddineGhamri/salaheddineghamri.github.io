use crate::routers::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(target_arch = "wasm32")]
#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="font-oswald bg-zinc-50">
          <div class="relative mx-auto py-4 h-16 flex w-full items-center sm:px-6 lg:px-8">
              <div class="grow h-16 flex items-center">

                // flex_1: logo
                <div class ="flex-1">
                <div class="flex flex-0 items-center text-[#0C0C0C]">
                    <img class="h-24 w-auto" src="./assets/salah_logo.svg" alt="my logo"/>
                    <div class="hidden flex-col flex-0 sm:flex">
                        <p class="font-raleway font-bold text-black text-l">{"Elevating Possibilities"}</p>
                        <p class="font-inter text-gray-600 text-xs">{"One Line at a Time"}</p>
                    </div>
                </div>
                </div>

                // flex_2: navigation elements
                <div class= "flex-1 justify-center">
                  <div class="flex justify-center items-center">
                    //<a href="#" class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Home"}</a>
                    <a class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                        <Link<Route> to={Route::Home}>
                        {"Home"}
                        </Link<Route>>
                    </a>
                    <a class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                        <Link<Route> to={Route::Blog}>
                        {"Blog"}
                        </Link<Route>>
                    </a>
                    <a class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                        <Link<Route> to={Route::Projects}>
                        {"Projects"}
                        </Link<Route>>
                    </a>
                  </div>
                </div>

                // flex_3: contact me
                <div class="flex-1 justify-end">
                    <div class="justify-end flex">
                    <a class="whitespace-nowrap text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
                        <Link<Route> to={Route::ContactMe}>
                        {"Contact me"}
                        </Link<Route>>
                    </a>
                    </div>
                </div>

            </div>
          </div>
        </nav>
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="font-oswald bg-zinc-50">
            <div class="relative mx-auto py-4 h-16 flex w-full items-center sm:px-6 lg:px-8">
                <div class="grow h-16 flex items-center">
                    <div class="flex-1 justify-center">
                        <div class="flex justify-center items-center">
                            <a href="/posts/index.html" class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Home"}</a>
                        </div>
                    </div>

                </div>
            </div>
        </nav>
    }
}
