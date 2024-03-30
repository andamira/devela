// devela::exec::async
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

/* always compiled */

mod coroutine;
mod reexports;

#[allow(unused_imports)] // coroutine
pub use {coroutine::*, reexports::*};

/* feature-gated */

#[cfg(all(not(feature = "safe_exec"), feature = "unsafe_async"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_async")))]
mod noop;

#[cfg(all(not(feature = "safe_exec"), feature = "unsafe_async"))]
pub use noop::TaskWakerNoop;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)] // coroutine
    pub use super::{coroutine::*, reexports::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(all(not(feature = "safe_exec"), feature = "unsafe_async"))]
    pub use super::noop::TaskWakerNoop;
}
