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
mod ext;
mod reexports;
mod waker;

#[allow(unused_imports)]
pub use {coroutine::*, ext::*, reexports::*, waker::*};

/* feature-gated */

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
mod block;

#[allow(unused_imports)]
#[cfg(feature = "std")]
pub use block::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{coroutine::*, ext::*, reexports::*, waker::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::block::*;
}
