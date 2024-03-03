use yew::prelude::*;

// 2. we create a parser for index.htmls in the posts
// 3. collect all tags
// posts_to_view are only posts that has the selected tag
// posts is a function that return html not a vector of htmls

// output of the parser
struct Post {
    title: String,
    description: String,
    image: String,
    date: String,
    tags: String,
}

//TODO fn posts(tags, all_posts) -> html

#[function_component]
pub fn Blog() -> Html {

    //TODO we need function here to define what posts to show
    let posts_to_view: Vec<Post> = vec![
               Post {
                   title: "How to create something".to_string(),
                   description: "This is coool or what ? ".to_string(),
                   image: "/posts/images/flowers.jpg".to_string(),
                   date: "2024-03-02".to_string(),
                   tags: "robotics".to_string(),
                    },
               Post {
                   title: "How to create something gggggggggggggggggg".to_string(),
                   description: "This is coool or what ? ".to_string(),
                   image: "/posts/images/flowers.jpg".to_string(),
                   date: "2024-03-02".to_string(),
                   tags: "robotics".to_string(),
                    },];

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
