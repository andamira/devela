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
mod coroutine;
mod reexports;

/* re-exports */

// always compiled, non-public
pub use {coroutine::*, reexports::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{coroutine::*, reexports::*};
}
