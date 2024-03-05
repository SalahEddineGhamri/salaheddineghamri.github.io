use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tag_name: String,
    pub href: String,
    pub on_tag_click: Callback<String>,
}

#[function_component]
pub fn Tag(props: &Props) -> Html {
    let tag_name = props.tag_name.clone();

    // change callback from string to event
    let on_click = props.on_tag_click.reform(move |_| tag_name.clone());

    html! {
        <a href= { props.href.clone() } onclick={on_click.clone()} class = "">{ &props.tag_name }</a>
    }
}
