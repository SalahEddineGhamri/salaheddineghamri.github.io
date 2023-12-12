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
            <nav class="bg-zinc-50">
              <div class="relative mx-auto py-4 h-24 flex w-full items-center sm:px-6 lg:px-8">
                  <div class="grow flex justify-between items-center sm:justify-start">

                    // logo
                    <div class ="flex-1">
                    <div class="flex justify-start">
                        <img class="h-16 w-auto " src="./assets/salah_logo.png" alt="my logo"/>
                    </div>
                    </div>

                    // navigation elements
                    <div class= "flex-1">
                      <div class="flex justify-center items-center space-x-4">
                        <a href="#" class="bg-gray-900 text-white rounded-md px-3 py-2 text-sm font-medium" aria-current="page">{"Home"}</a>
                        <a href="#" class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Blog"}</a>
                        <a href="#" class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">{"Projects"}</a>
                      </div>
                    </div>

                    // contact me
                    <div class="flex-1">
                        <div class="flex justify-end">
                            <a href="#" class="text-gray-500 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium" aria-current="page">{"Contact me"}</a>
                        </div>
                    </div>

                </div>
              </div>
            </nav>
        }
    }
}
