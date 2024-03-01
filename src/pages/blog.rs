use yew::prelude::*;

// 1. we create the poste itself: image - title - summery

// TODO create the layout for the blog page, count how much page we have then act on it
// TODO create a function that generates html on the fly

/*
we need filters for: categories - tags
we need for each blog icons:
    - image
    - title
    - summery
*/

#[function_component]
pub fn Blog() -> Html {
    html!{
    <>
        <h1 class="my-48 text-center mr-4 text-3xl font-bold">{"No blog yet, because we need a rust 'Markdown to Html converter'"}</h1>
    </>
    }
}
