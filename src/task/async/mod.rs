// devela::task::async
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

/* contains always compiled items */

mod reexports;

/* feature-gated */

#[cfg(all(feature = "unsafe_task", feature = "alloc"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "unsafe_task", feature = "alloc")))
)]
pub mod coroutine;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(all(feature = "unsafe_task", feature = "alloc"))]
pub use coroutine::*;

// re-export private sub-modules
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(all(feature = "unsafe_task", feature = "alloc"))]
    pub use super::coroutine::*;
}
