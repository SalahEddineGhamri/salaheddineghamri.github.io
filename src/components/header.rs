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
            <div class="justify-center relative mx-auto py-20 flex items-center sm:px-6 lg:px-8">
                // description
                <div class="">
                    <h1 class="mr-4 text-3xl font-bold">{"Salah Eddine Ghamri"}</h1>
                    <p class="text-gray-600">{"a little description about myself"}</p>
                    <p class="mr-4 text-black">{"Here are some ideas to get me started but no time to fill them:"}
                                               <br/>
                                               {"🔭 I’m currently working on ..."}
                                               <br/>
                                               {"💬 Ask me about .."}
                                               <br/>
                                               <br/>
                                               <br/>
                                               <br/>
                                               <br/>
                                               {"👯 I’m looking to collaborate on ..."} </p>
                </div>
                // profile photo
                <div class="mb-4">
                  <div class="rounded-full overflow-hidden">
                    <img src="./assets/profile.jpg" alt="Profile Photo" class="h-96 w-96 object-cover"/>
                  </div>
                </div>
            </div>
        }
    }
}
