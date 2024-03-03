use yew::prelude::*;
use select::document::Document;
use select::predicate::{Attr, Name, Class};
use include_dir::{include_dir, Dir};

use gloo_console::log;
use wasm_bindgen::JsValue;

const POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/posts/");

struct Post {
    title: String,
    description: String,
    image: String,
    date: String,
    tags: Vec<String>,
}

impl Post {
    fn new(title: String, description: String, image: String, date: String, tags: Vec<String>) -> Self {
        Self {
            title,
            description,
            image,
            date,
            tags,
        }
    }
}

//TODO: id of a post is the name of folder
//TODO: change the black color

fn parse_posts() -> Vec<Post> {

    let mut posts = Vec::new();
    let dirs = POSTS_DIR.dirs();

    for dir in dirs {
        let option_dir_name = dir.path().to_str();
        if let Some(dir_name) = option_dir_name {
            if dir_name.starts_with("test"){
                let file = dir.get_file(format!("{}/index.html", dir_name)).unwrap();
                let document = Document::from(file.contents_utf8().unwrap());
                let title = document.find(Name("title")).next().map(|n| n.text()).unwrap_or_default().to_string();
                let image = document.find(Attr("property", "og:image")).next().map(|n| n.attr("content").unwrap_or_default()).unwrap_or_default().to_string();
                let description = document.find(Attr("name", "description")).next().map(|n| n.attr("content").unwrap_or_default()).unwrap_or_default().to_string();
                let date = document.find(Attr("name", "publish-date")).next().map(|n| n.attr("content").unwrap_or_default()).unwrap_or_default().to_string();
                let tags = document.find(Class("post-tag")).map(|tag| tag.text()).collect::<Vec<_>>();

                let post = Post{title: title,
                                description: description,
                                image: image,
                                date: date,
                                tags: tags,
                                };

                posts.push(post);
            }
        }
    }
    posts
}

//TODO make separate file
#[function_component]
pub fn TagSelector() -> Html {
    html! {
        <a class = "">{" tag1 "}</a>
    }
}

//TODO make separate file
#[function_component]
pub fn PostsGrid() -> Html {

    let posts: Vec<Html> = vec![];

    html!{
        <span class = "grid text-blue-500 gap-2 auto-cols-auto auto-rows-auto row-auto place-items-center">{posts}</span>
    }
}

#[function_component]
pub fn Blog() -> Html {
    // tags sends the tags_name to state through callback
    // tag_name determins posts to view
    // send them as attrib to grib element
    let path = "posts";
    let base_url = "salaheddineghamri.github.io";

    let posts_to_view = parse_posts();

    let posts: Vec<Html> = posts_to_view
        .iter()
        .map(|post: &Post| {
            html! {
                <div class = "aspect-auto flex flex-col bg-white rounded-lg shadow-lg p-4 w-72">
                    <img class = "rounded-lg" src={post.image.clone()}/>
                    <div class = "p-4">{&post.title}</div>
                    <div class = "text-sm text-gray-600  font-inter font-normal">{&post.description}</div>
                    <div class = "text-sm text-gray-400  font-inter font-normal">{&post.date}</div>
                </div>
            }
        })
        .collect();
    html!{
    <>
        <div class = " container mx-auto self-center flex flex-col justify-self-center items-center p-8 w-3/4 md:flex-row">
            // TODO: this a loop of tags
            <div class = "bg-white flex flex-col rounded-lg shadow-lg w-64 h-auto p-8">
                <a class = "">{" tag1 "}</a>
                <a class = "">{" tag2 "}</a>
            </div>
            <div class = "p-10 justify-self-center">
                <span class = "grid text-blue-500 gap-2 auto-cols-auto auto-rows-auto row-auto place-items-center">{posts}</span>
            </div>
        </div>
    </>
    }
}
