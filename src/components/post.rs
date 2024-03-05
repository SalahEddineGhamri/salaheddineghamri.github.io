use yew::prelude::*;

use gloo_console::log;
use wasm_bindgen::JsValue;

#[derive(Properties, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image: String,
    pub date: String,
    pub tags: Vec<String>,
}

impl Post {
    pub fn new(
        id: i32,
        title: String,
        description: String,
        image: String,
        date: String,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            image,
            date,
            tags,
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[function_component]
pub fn PostComponent(post: &Post) -> Html {
    html! {
        <div class = "aspect-auto flex flex-col bg-white rounded-lg shadow-lg p-4 w-72">
            <img class = "rounded-lg" src={post.image.clone()}/>
            <div class = "p-4">{&post.title}</div>
            <div class = "text-sm text-gray-600  font-inter font-normal">{&post.description}</div>
            <div class = "text-sm text-gray-400  font-inter font-normal">{&post.date}</div>
        </div>
    }
}
