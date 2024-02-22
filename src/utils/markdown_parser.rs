use yew::prelude::*;

// TODO  function that reads some .md file and return html

pub fn generate_article() -> Html {
    let paragraph_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    html!{
        <div>
            <p>{paragraph_content}</p>
        </div>
    }
}
