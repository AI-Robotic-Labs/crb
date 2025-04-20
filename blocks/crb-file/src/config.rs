use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context};
use crb::superagent::{OnRequest, Request};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::path::PathBuf;
use tokio::fs;
use toml::Value;

pub trait Config: DeserializeOwned + Send + 'static {
    // TODO: const NAMESPACE
}

impl<T> Config for T where T: DeserializeOwned + Send + 'static {}

pub struct ConfigAgent {}

impl Agent for ConfigAgent {
    type Context = AgentSession<Self>;
}

pub struct ReadConfig<T = Value> {
    pub path: PathBuf,
    pub _type: PhantomData<T>,
}

impl<T: Config> Request for ReadConfig<T> {
    type Response = T;
}

#[async_trait]
impl<T: Config> OnRequest<ReadConfig<T>> for ConfigAgent {
    async fn on_request(&mut self, msg: ReadConfig<T>, _ctx: &mut Context<Self>) -> Result<T> {
        let content = fs::read_to_string(&msg.path).await?;
        // TODO: Detect file type, override values from a namespace
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}
