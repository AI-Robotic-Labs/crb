//! The crate includes a universal channel and
//! a function for initiating asynchronous activities.

#[cfg(not(any(feature = "std", feature = "web")))]
compile_error!("Environment feature must be activated: std or web");

#[cfg(all(feature = "std", feature = "web"))]
compile_error!("Only one environment type must be selected: std or web");

#[cfg(all(feature = "std", not(feature = "web")))]
pub use crb_core_std::*;

#[cfg(all(feature = "web", not(feature = "std")))]
pub use crb_core_web::*;

pub use futures;
pub use tokio::sync::{self, mpsc, watch};

mod types;
pub use types::*;
