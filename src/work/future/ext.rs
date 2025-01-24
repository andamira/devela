// devela::work::future::ext
//
//!
//

#[cfg(feature = "std")]
use {
    crate::{
        future_block, Future, FuturePending, FuturePollFn, FutureReady, TaskContext, TaskPoll,
    },
    std::future::{pending, poll_fn, ready},
};

impl<F: Future> ExtFuture for F {}

/// Extension trait providing additional methods for [`Future`]s.
pub trait ExtFuture: Future {
    /// Blocks the thread until the future is ready.
    ///
    /// # Example
    /// ```
    /// use devela::ExtFuture as _;
    ///
    /// let future = async {};
    /// let result = future.block_on();
    /// ```
    ///
    /// # Derived work
    #[doc = include_str!("./block/MODIFICATIONS.md")]
    #[rustfmt::skip]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
    fn block_on(self) -> Self::Output where Self: Sized { future_block(self) }

    /// Creates a future which never resolves.
    #[rustfmt::skip]
    fn pending<T>() -> FuturePending<T> { pending() }

    /// Creates a future that wraps a `function` returning [`TaskPoll`].
    #[rustfmt::skip]
    fn poll_fn<T, F>(function: F) -> FuturePollFn<F>
    where F: FnMut(&mut TaskContext<'_>) -> TaskPoll<T> { poll_fn(function) }

    /// Creates a future that is immediately ready with a `value`.
    #[rustfmt::skip]
    fn ready<T>(value: T) -> FutureReady<T> { ready(value) }
}
