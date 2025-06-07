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
            <div class = "max-w-6xl mx-auto text-[#0C0C0C]">
            <div class = "flex flex-col-reverse px-10 py-10 flex-col sm:flex-row sm:px-30 lg:px-56">
                // description
                <div class = "px-6 self-center justify-self-start flex-1 flex flex-col pt-6 items-center sm:items-start sm:pt-4">
                    <div class = "justify-self-center py-4 sm:justify-self-center sm:py-6">
                        <h1 class="text-xl sm:text-3xl font-bold">{"Salah Eddine Ghamri"}</h1>
                        <p class="text-gray-600 text-l sm:text-s">{"Researcher & Software Engineer"}</p>
                    </div>
                    <p class="font-inter text-black text-justify">
                                               {"My background spans electronics, software development, and research,
                                               with a focus on mobile robotics and autonomous systems. I have worked on
                                               autonomous behavior and transportation technology. Currently, I
                                               contribute to automotive projects, using programming languages like C++,
                                               Python, and Rust, and working closely with hardware such as SoCs and
                                               ECUs. I develop software aimed at improving system reliability and
                                               performance. I also write technical blogs to share practical insights on
                                               emerging technologies. I am motivated by collaboration and continuous learning, aiming to support meaningful progress in embedded and automotive software development."}
                                               </p>
                    // linkedin button
                    <div class = "flex py-6 space-x-2 items-center">
                    <a href = "https://de.linkedin.com/in/salaheddineghamri/en">
                    <button
                      type = "button"
                      class = "mb-2 inline-block rounded px-6 py-2.5 text-xs font-medium uppercase leading-normal text-white shadow-md transition duration-150 ease-in-out hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg"
                      style = "background-color: #0077b5">
                      <svg
                        xmlns = "http://www.w3.org/2000/svg"
                        class = "h-4 w-4"
                        fill = "currentColor"
                        viewBox = "0 0 24 24">
                        <path
                          d = "M4.98 3.5c0 1.381-1.11 2.5-2.48 2.5s-2.48-1.119-2.48-2.5c0-1.38 1.11-2.5 2.48-2.5s2.48 1.12 2.48 2.5zm.02 4.5h-5v16h5v-16zm7.982 0h-4.968v16h4.969v-8.399c0-4.67 6.029-5.052 6.029 0v8.399h4.988v-10.131c0-7.88-8.922-7.593-11.018-3.714v-2.155z" />
                      </svg>
                    </button>
                    </a>

                    // github button
                    <a href="https://github.com/SalahEddineGhamri">
                    <button
                      type="button"
                      class="mb-2 inline-block rounded px-6 py-2.5 text-xs font-medium uppercase leading-normal text-white shadow-md transition duration-150 ease-in-out hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg"
                      style="background-color: #333">
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4"
                        fill="currentColor"
                        viewBox="0 0 24 24">
                        <path
                          d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                      </svg>
                    </button>
                    </a>
                    </div>
                </div>

                // profile photo
                <div class="flex-0 self-center">
                  <div class="rounded-full overflow-hidden">
                    <img src="./assets/profile.jpg" alt="Profile Photo" class="h-40 w-40 sm:h-80 sm:w-80 object-cover"/>
                  </div>
                </div>
            </div>
            </div>
        }
    }
}
