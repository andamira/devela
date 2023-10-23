// devela::task::async
//
//! Asynchrony, extends
//! `std::{`[`future`][std::future],
//! [`task`][std::task]`}`.
//!
//! See also the fundamental [`async`] and [`await`] keywords and the
//! [async book](https://rust-lang.github.io/async-book/).
//!
//! [`async`]: https://doc.rust-lang.org/std/keyword.async.html
//! [`await`]: https://doc.rust-lang.org/std/keyword.await.html
//

/* always compiled for internal use */

/* only compiled with the `task` feature */

#[cfg(all(feature = "unsafe_task", feature = "alloc"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "unsafe_task", feature = "alloc")))
)]
pub mod coroutine;
#[cfg(all(feature = "unsafe_task", feature = "alloc"))]
pub use coroutine::*;

/* re-exports */

mod reexports;
pub use reexports::*;

pub(crate) mod all {
    pub use super::reexports::*;

    #[cfg(all(feature = "unsafe_task", feature = "alloc"))]
    pub use super::coroutine::*;
}
