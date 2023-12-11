use yew::prelude::*;
use crate::components::{Header, Navbar, Row, Footer};

mod components;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
    <>
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>

        <Header />
        <Navbar />
        <Row />
        <Footer />
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
