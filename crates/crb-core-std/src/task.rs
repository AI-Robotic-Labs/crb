//! This module provides a simple wrapper
//! around the `tokio::task` module.

pub use tokio::task::{spawn_blocking, JoinHandle};
pub use tokio::{spawn, task::spawn_local};
