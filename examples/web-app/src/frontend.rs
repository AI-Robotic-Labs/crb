use crate::web_app::WebApp;
use anyhow::Result;
use crb::agent::{Agent, AgentSession, Address, DoSync, Next, Standalone};

pub struct Frontend;

impl Frontend {
    pub fn new() -> Self {
        Self
    }
}

impl Standalone for Frontend {}

impl Agent for Frontend {
    type Context = AgentSession<Self>;
    type Link = Address<Self>;

    fn begin(&mut self) -> Next<Self> {
        // Important to check `DoSync` works
        Next::do_sync(Bootstrap)
    }
}

struct Bootstrap;

impl DoSync<Bootstrap> for Frontend {
    fn once(&mut self, _: &mut Bootstrap) -> Result<Next<Self>> {
        log::info!("DoSync works!");
        yew::Renderer::<WebApp>::new().render();
        Ok(Next::events())
    }
}
