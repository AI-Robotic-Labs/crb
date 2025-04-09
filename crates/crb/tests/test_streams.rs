use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, ManagedContext, Next, OnEvent, Standalone};
use crb::superagent::StreamSession;
use futures::stream::{StreamExt, once};

struct TestStreams {
    counter: usize,
}

impl TestStreams {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Standalone for TestStreams {}

impl Agent for TestStreams {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Consume)
    }
}

struct Consume;

#[async_trait]
impl DoAsync<Consume> for TestStreams {
    async fn handle(&mut self, _: Consume, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let stream = once(async { () }).boxed();
        ctx.consume(stream);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<()> for TestStreams {
    async fn handle(&mut self, _: (), ctx: &mut Context<Self>) -> Result<()> {
        self.counter += 1;
        if self.counter < 5 {
            ctx.do_next(Next::do_async(Consume));
        } else {
            ctx.shutdown();
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_streams() -> Result<()> {
    let mut addr = TestStreams::new().spawn();
    addr.join().await?;
    Ok(())
}
