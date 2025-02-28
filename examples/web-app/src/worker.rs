use crb::agent::{Agent, Next, Standalone};
use crb::superagent::StreamSession;

pub struct Worker {
}

impl Worker {
    pub fn new() -> Self {
        Self { }
    }
}

impl Standalone for Worker {
}

impl Agent for Worker {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        log::info!("Worker STARTED!");
        Next::events()
    }
}
