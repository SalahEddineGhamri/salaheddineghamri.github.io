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
            <div class="flex flex-col-reverse justify-center mx-auto my-auto px-4 py-10 flex-col sm:flex-row sm:px-10 lg:px-8">
                // description
                <div class="pt-6 sm:pt-4">
                    <h1 class="mr-4 text-3xl font-bold">{"Salah Eddine Ghamri"}</h1>
                    <p class="text-gray-600">{"a little description about myself"}</p>
                    <p class="font-inter mr-4 text-black">{"Here are some ideas to get me started but no time to fill them:"}
                                               <br/>
                                               {"🔭 I’m currently working on ..."}
                                               <br/>
                                               {"💬 Ask me about .."}
                                               <br/>
                                               <br/>
                                               {"👯 I’m looking to collaborate on ..."}
                                               <br/>
                                               </p>
                </div>

                // profile photo
                <div class="self-center">
                  <div class="rounded-full overflow-hidden">
                    <img src="./assets/profile.jpg" alt="Profile Photo" class="h-40 w-40 sm:h-96 sm:w-96 object-cover"/>
                  </div>
                </div>
            </div>
        }
    }
}
