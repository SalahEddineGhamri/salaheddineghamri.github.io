use yew::prelude::*;

use crate::components::post;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub posts: Vec<post::Post>,
}

#[function_component]
pub fn Posts(props: &Props) -> Html {
    let posts: Vec<Html> = props.posts
        .iter()
        .map(|post: &post::Post| html! { <post::PostComponent title={post.title.clone()} date={post.date.clone()} description={post.description.clone()} tags={post.tags.clone()} image={post.image.clone()} id={post.id} path={post.path.clone()}/>})
        .collect();

    html! {
        <span class = "grid text-blue-500 gap-2 auto-cols-auto auto-rows-auto row-auto place-items-center">{posts}</span>
    }
}
