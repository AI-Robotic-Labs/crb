//! This module provides a simple wrapper
//! around the `tokio::task` module.

pub use tokio::task::{JoinHandle, spawn_blocking};
pub use tokio::{spawn, task::spawn_local};
