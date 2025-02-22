// devela::work::future::ext
//
//!
//

#[cfg(feature = "std")]
#[cfg(not(feature = "dep_portable_atomic_util"))]
use crate::future_block;

use {
    crate::{Future, FuturePending, FuturePollFn, FutureReady, TaskContext, TaskPoll},
    ::core::future::{pending, poll_fn, ready},
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
    /// # Features
    /// This method is only available if the `dep_portable_atomic_util` feature is **disabled**,
    /// because its `Arc` type can't be used as a `self` type.
    /// See [arbitrary_self_types](https://github.com/rust-lang/rust/issues/44874).
    ///
    #[doc = crate::doc_!(vendor: "pollster")]
    #[rustfmt::skip]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
    #[cfg(not(feature = "dep_portable_atomic_util"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(not(feature = "dep_portable_atomic_util"))))]
    // WAIT: [arbitrary_self_types](https://github.com/rust-lang/rust/pull/135881)
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
