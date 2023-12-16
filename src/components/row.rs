use yew::prelude::*;

pub struct Row;

impl Component for Row {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Row
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
                <div class="side">{"..."}</div>
            </div>
        }
    }
}
