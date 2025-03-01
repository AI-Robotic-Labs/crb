//! The crate includes a universal channel and
//! a function for initiating asynchronous activities.

/*
#[cfg(feature = "std")]
use crb_core_std as crb_core_impl;

#[cfg(feature = "wasm")]
use crb_core_wasm as crb_core_impl;
*/

pub use futures;
pub use uuid;

#[cfg(not(target_arch = "wasm32"))]
mod std_runtime;
#[cfg(not(target_arch = "wasm32"))]
pub use std_runtime::*;

#[cfg(target_arch = "wasm32")]
mod web_runtime;
#[cfg(target_arch = "wasm32")]
pub use web_runtime::*;

pub use tokio::sync::{self, mpsc, watch};

mod types;
pub use types::*;
