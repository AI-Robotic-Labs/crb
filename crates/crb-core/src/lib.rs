//! The crate includes a universal channel and
//! a function for initiating asynchronous activities.

#[cfg(not(target_arch = "wasm32"))]
use crb_core_std as crb_core_impl;

#[cfg(target_arch = "wasm32")]
use crb_core_web as crb_core_impl;

pub use crb_core_impl::*;
pub use futures;
pub use tokio::sync::{self, mpsc, watch};

mod types;
pub use types::*;
