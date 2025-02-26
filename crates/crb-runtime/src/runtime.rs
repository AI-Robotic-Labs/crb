//! A runtime for composable blocks.

use crate::context::ReachableContext;
use crate::interruptor::{InterruptionLevel, Interruptor};
use async_trait::async_trait;
use std::ops::DerefMut;

/// The `Runtime` trait implements an asynchronous task that can be interrupted.
///
/// Consequently, this trait must implement the `Send` marker trait, allowing the runtime
/// to move between different threads of the asynchronous reactor, as well as the `'static` lifetime,
/// which enables encapsulating the runtime in a `Box`.
#[async_trait]
pub trait Runtime: Send + 'static {
    /// The `get_interruptor` method returns an instance of an interruptor.
    ///
    /// The reutrned value implements the `Interruptor` trait and can be used by any system
    /// launching the agent's runtime to interrupt its execution. The implementation of the `Interruptor`
    /// depends on the entity implementing the `Runtime`.
    fn get_interruptor(&mut self) -> Box<dyn Interruptor>;

    /// The `interruption_level` method returns the default level at which the runtime
    /// should be interrupted.
    ///
    /// Since the interruption system has different levels to allow for a graceful termination
    /// of the runtime, even if it is deeply nested in coroutines, this enables a smooth shutdown
    /// starting from the deepest level.
    fn interruption_level(&self) -> InterruptionLevel {
        InterruptionLevel::EVENT
    }

    /// The asynchronous `routine` method is the primary execution method of the `Runtime`.
    ///
    /// It is called by the activity that runs (owns) the `Runtime`.
    async fn routine(&mut self);
}

pub trait InteractiveRuntime: Runtime {
    /// Type of the composable block's contenxt.
    type Context: ReachableContext;

    fn address(&self) -> <Self::Context as ReachableContext>::Address;
}

#[async_trait]
impl Runtime for Box<dyn Runtime> {
    fn get_interruptor(&mut self) -> Box<dyn Interruptor> {
        self.deref_mut().get_interruptor()
    }

    async fn routine(&mut self) {
        self.deref_mut().routine().await
    }
}
