use yew::prelude::*;
use yew_router::prelude::*;
use crate::routers::Route;

//TODO: detect night mode and adjust accordingly
#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="bg-zinc-50">
          <div class="relative mx-auto py-4 h-16 flex w-full items-center sm:px-6 lg:px-8">
              <div class="grow flex justify-between items-center sm:justify-start">

                // logo
                <div class ="flex-0">
                <div class="flex flex-1 items-center">
                    <img class="h-24 w-auto" src="./assets/salah_logo.svg" alt="my logo"/>
                    <div class="hidden flex-col sm:flex">
                        <p class="font-lato font-bold text-black text-l">{"Elevating Possibilities"}</p>
                        <p class="font-telex text-gray-600 text-xs">{"One Line at a Time"}</p>
                    </div>
                </div>
                </div>

                // navigation elements
                <div class= "flex-1">
                  <div class="flex justify-center items-center space-x-4">
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

                // contact me
                <div class="flex-0 flex-shrink-0">
                    <div class="flex flex-nowrap justify-end flex-shrink-0">
                    <a class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">
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
