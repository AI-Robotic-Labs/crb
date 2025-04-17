use crb::agent::{Agent, AgentSession, Context};
use crb::superagent::{Request, OnRequest};
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use toml::Value;
use tokio::fs;

pub struct ConfigAgent {
}

impl Agent for ConfigAgent {
    type Context = AgentSession<Self>;
}

pub struct ReadConfig {
    pub path: PathBuf,
}

impl Request for ReadConfig {
    type Response = Value;
}

#[async_trait]
impl OnRequest<ReadConfig> for ConfigAgent {
    async fn on_request(&mut self, msg: ReadConfig, _ctx: &mut Context<Self>) -> Result<Value> {
        let content = fs::read_to_string(&msg.path).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}
