// devela::work::async
//
//! Asynchrony, extends `std::{`[`future`], [`task`]`}`.
//!
//! [`future`]: std::future
//! [`task`]: std::task
//!
//! See also the fundamental [`async`] and [`await`] keywords and the
//! [async book](https://rust-lang.github.io/async-book/).
//!
//! [`async`]: https://doc.rust-lang.org/std/keyword.async.html
//! [`await`]: https://doc.rust-lang.org/std/keyword.await.html
//

/* modules */

// always compiled, non public
mod reexports;

// feature gated, public
#[cfg(all(
    not(feature = "safe_work"),
    feature = "unsafe_async",
    feature = "alloc"
))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "unsafe_async", feature = "alloc")))
)]
pub mod coroutine;

/* re-exports */

// always compiled, non-public
pub use reexports::*;

// feature-gated, public
#[doc(no_inline)]
#[cfg(all(
    not(feature = "safe_work"),
    feature = "unsafe_async",
    feature = "alloc"
))]
pub use coroutine::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::reexports::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(all(
        not(feature = "safe_work"),
        feature = "unsafe_async",
        feature = "alloc"
    ))]
    pub use super::coroutine::*;
}
