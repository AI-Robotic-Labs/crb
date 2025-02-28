use yew::{html, Component, Context, Html};

pub struct WebApp {}

impl Component for WebApp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>{ "Web App, crabs!" }</p>
        }
    }
}
