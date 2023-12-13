use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Home, Blog, Projects, ContactMe};

// all routes defined here
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/projects")]
    Projects,
    #[at("/contact_me")]
    ContactMe,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route{
        Route::Home => html!{<Home/>},
        Route::Blog => html!{<Blog/>},
        Route::Projects => html!{<Projects/>},
        Route::ContactMe => html!{<ContactMe/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
