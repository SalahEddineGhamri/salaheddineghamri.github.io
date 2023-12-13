use yew::prelude::*;
use crate::components::{Header, Row};

#[function_component]
pub fn Home() -> Html {
    html!{
    <>
        <Header />
        <Row />
    </>
    }
}
