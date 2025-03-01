use crate::worker::Worker;
use crb::agent::{Address, Equip, Standalone};
use crb::core::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use yew::{html, Component, Context, Html};

pub enum Message {
    Add,
    Reset,
}

pub struct WebApp {
    crabs: usize,
    _worker: Address<Worker>,
}

impl Component for WebApp {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let stream = UnboundedReceiverStream::new(rx);
        ctx.link().send_stream(stream);
        let worker = Worker::new(tx).spawn().equip();
        Self {
            crabs: 0,
            _worker: worker,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Message) -> bool {
        match msg {
            Message::Add => {
                self.crabs += 1;
            }
            Message::Reset => {
                self.crabs = 0;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Message::Reset);
        let crabs: String = std::iter::repeat("🦀").take(self.crabs).collect();
        html! {
            <div>
                <p>{ "Web App, crabs!" }</p>
                <button {onclick}>{ "Reset!" }</button>
                <p>{ crabs }</p>
            </div>
        }
    }
}
