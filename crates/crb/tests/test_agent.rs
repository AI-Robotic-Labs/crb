use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, OnEvent, Standalone};
use derive_more::{Deref, DerefMut, From};

struct PrinterAgent;

impl Standalone for PrinterAgent {}

impl Agent for PrinterAgent {
    type Context = AgentSession<Self>;
    type Link = Printer;
}

struct Print(pub String);

#[async_trait]
impl OnEvent<Print> for PrinterAgent {
    async fn handle(&mut self, event: Print, _ctx: &mut Context<Self>) -> Result<()> {
        println!("{}", event.0);
        Ok(())
    }
}

#[derive(Deref, DerefMut, From)]
struct Printer {
    address: Address<PrinterAgent>,
}

impl Printer {
    fn print(&self, msg: &str) -> Result<()> {
        let print = Print(msg.into());
        self.address.event(print)?;
        Ok(())
    }
}

#[tokio::test]
async fn test_agent() -> Result<()> {
    let mut printer = PrinterAgent.spawn();
    printer.print("Hello, Agent!")?;
    printer.interrupt()?;
    printer.join().await?;
    Ok(())
}
