pub use web_time::Instant;

#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
pub use tokio_with_wasm::{spawn, spawn_local, task::JoinHandle};
