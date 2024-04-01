// devela::data::exec::async::ext
//
//!
//

#[cfg(feature = "std")]
use super::future_block;
use super::Future;

impl<F: Future> ExtFuture for F {}

/// Extension trait providing additional methods for [`Future`]s.
pub trait ExtFuture: Future {
    /// Blocks the thread until the future is ready.
    ///
    /// See also the [`future_block`] function.
    ///
    /// # Example
    /// ```
    /// use devela::ExtFuture as _;
    ///
    /// let future = async {};
    /// let result = future.block_on();
    /// ```
    #[rustfmt::skip]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
    fn block_on(self) -> Self::Output where Self: Sized { future_block(self) }
}
