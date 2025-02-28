//! The module with parts that are compatible with both WASM
//! and non-WASM environments.

pub use tokio::sync::{self, mpsc, oneshot, watch};

/// A compatible time module.
pub mod time {
    pub use crate::crb_core_impl::Instant;
    pub use crate::crb_core_impl::time::{timeout};
    pub use tokio::time::{sleep_until, Duration, Sleep};
}

pub use crate::crb_core_impl::{spawn, spawn_local, JoinHandle};
