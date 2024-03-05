use crate::components::{Header, Row};
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
    <>
        <Header />
        <Row />
    </>
    }
}
