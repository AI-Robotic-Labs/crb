//! CRB core for the Web environment.

pub mod task;
pub use task::*;

pub mod time {
    pub use core::time::Duration;
    pub use wasmtimer::std::Instant;
    pub use wasmtimer::tokio::*;
}

pub use uuid;
