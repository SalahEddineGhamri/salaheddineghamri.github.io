use yew::prelude::*;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use include_dir::{include_dir, Dir};

use gloo_console::log;
use wasm_bindgen::JsValue;

use crate::components::post;
use crate::components::posts;
use crate::components::tag;

const POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/posts/");

//TODO: change the black color

fn parse_posts() -> (Vec<post::Post>, Vec<String>) {

    let mut posts = Vec::new();
    let mut all_tags = Vec::<String>::new();

    let dirs = POSTS_DIR.dirs();

    for dir in dirs {
        let option_dir_name = dir.path().to_str();
        if let Some(dir_name) = option_dir_name {
            if dir_name.starts_with("post"){
                let file = dir.get_file(format!("{}/index.html", dir_name)).unwrap();
                let document = Document::from(file.contents_utf8().unwrap());
                let title = document.find(Name("title")).next().map(|n| n.text()).unwrap_or_default().to_string();
                let image = document.find(Attr("property", "og:image")).next().map(|n| n.attr("content").unwrap_or_default()).unwrap_or_default().to_string();
                let description = document.find(Attr("name", "description")).next().map(|n| n.attr("content").unwrap_or_default()).unwrap_or_default().to_string();
                let date = document.find(Attr("name", "publish-date")).next().map(|n| n.attr("content").unwrap_or_default()).unwrap_or_default().to_string();

                let tags_vector = document.find(Class("post-tags").descendant(Name("a"))).map(|tag_item| tag_item.text()).collect::<Vec<String>>();
                let tags = tags_vector.clone();
                all_tags.extend(tags_vector);

                // TODO drive id from the name of page's folder
                // TODO drive link path

                let post = post::Post::new(0, title, description, image, date, tags);

                posts.push(post);
            }
        }
    }
    (posts, all_tags)
}


pub fn filter_posts(tag_name: &str, posts: Vec<post::Post>) -> Vec<post::Post> {
    posts.into_iter().filter(|post| post.has_tag(tag_name)).collect::<Vec<_>>()
}


#[function_component]
pub fn Blog() -> Html {
    let tag = use_state(|| "#test".to_string());

    let (all_posts, all_tags) = parse_posts();

    let posts_to_view = filter_posts(&tag, all_posts);

    let on_tag_click: Callback<String> = Callback::from(move |tag_name: String| {
        tag.set(tag_name);
    });

    let listed_tags: Vec<Html> = all_tags.iter().map(|tag_item: &String| html!{ <> <tag::Tag tag_name={tag_item.clone()} on_tag_click={on_tag_click.clone()}/> </>}).collect();

    html!{
        <>
            <div class = " container mx-auto self-center flex flex-col justify-self-center items-center p-8 w-3/4 md:flex-row">
                <div class = "bg-white flex flex-col rounded-lg shadow-lg w-64 h-auto p-8">{listed_tags}</div>
                <div class = "p-10 justify-self-center">
                    <posts::Posts posts={posts_to_view} />
                </div>
            </div>
        </>
    }
}
