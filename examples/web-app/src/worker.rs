use crate::web_app::Message;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent, Standalone};
use crb::core::mpsc;
use crb::superagent::{Interval, StreamSession, Tick};

pub struct Worker {
    interval: Interval,
    component: mpsc::UnboundedSender<Message>,
}

impl Worker {
    pub fn new(component: mpsc::UnboundedSender<Message>) -> Self {
        Self {
            interval: Interval::new(),
            component,
        }
    }
}

impl Standalone for Worker {}

impl Agent for Worker {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        log::info!("Worker STARTED!");
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Worker {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        log::info!("DoAsync works!");
        ctx.consume(self.interval.events()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Tick> for Worker {
    async fn handle(&mut self, _: Tick, _ctx: &mut Context<Self>) -> Result<()> {
        log::info!("Tick");
        self.component.send(Message::Add)?;
        Ok(())
    }
}
