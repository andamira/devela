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
mod waker;

#[allow(unused_imports)]
pub use {coroutine::*, reexports::*, waker::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{coroutine::*, reexports::*, waker::*};
}