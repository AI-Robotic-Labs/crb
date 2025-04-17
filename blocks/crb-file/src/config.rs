use crb::agent::{Agent, AgentSession};

pub struct ConfigAgent {
}

impl Agent for ConfigAgent {
    type Context = AgentSession<Self>;
}
