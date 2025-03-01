//! Task module for spawning async tasks
//! in WASM environment.

use futures::channel::oneshot;
use futures::future::{AbortHandle, Abortable, Aborted};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use thiserror::Error;

/// Error type returned when awaiting a spawned task.
#[derive(Error, Debug)]
pub enum JoinError {
    /// The task failed to send a result (e.g., the channel was dropped).
    #[error("The task is cancelled")]
    Canceled,
    /// The task was aborted.
    #[error("The task is aborted")]
    Aborted,
}

/// JoinHandle wraps a oneshot::Receiver for obtaining the task’s result,
/// along with an AbortHandle to allow the task to be aborted.
pub struct JoinHandle<T> {
    receiver: oneshot::Receiver<Result<T, Aborted>>,
    abort_handle: AbortHandle,
}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, JoinError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.receiver).poll(cx) {
            // The task completed and sent a result.
            Poll::Ready(Ok(result)) => Poll::Ready(match result {
                Ok(val) => Ok(val),
                Err(_) => Err(JoinError::Aborted),
            }),
            // The oneshot channel was closed before a value could be sent.
            Poll::Ready(Err(_)) => Poll::Ready(Err(JoinError::Canceled)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> JoinHandle<T> {
    /// Aborts the spawned task.
    pub fn abort(&self) {
        self.abort_handle.abort();
    }
}

/// Spawns a future as an abortable task using spawn_local.
/// This function is equivalent to tokio's spawn_local.
pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    // Create a oneshot channel for sending the result.
    let (sender, receiver) = oneshot::channel();
    // Create an AbortHandle and AbortRegistration pair.
    let (abort_handle, abort_registration) = AbortHandle::new_pair();

    // Wrap the future in Abortable so it can be aborted.
    let abortable_future = Abortable::new(future, abort_registration);

    // The wrapped future awaits completion or abortion, then sends its result.
    let wrapped_future = async move {
        let res = abortable_future.await;
        // Ignore errors if the receiver has been dropped.
        let _ = sender.send(res);
    };

    wasm_bindgen_futures::spawn_local(wrapped_future);

    JoinHandle {
        receiver,
        abort_handle,
    }
}

/// Spawns a future as an abortable task.
/// In WASM, spawn and spawn_local are equivalent.
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    spawn_local(future)
}

/// Spawns a blocking task by wrapping the blocking function in an async block.
/// In WASM this is equivalent to spawn_local since true blocking cannot be offloaded to another thread.
pub fn spawn_blocking<F, R>(blocking_func: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + 'static,
    R: 'static,
{
    spawn_local(async move { blocking_func() })
}
