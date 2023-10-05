// devela::future
//
//! Asynchronous functionality, extends [`core::future`].
//!
//! See also the fundamental [`async`] and [`await`] keywords and the
//! [async book](https://rust-lang.github.io/async-book/).
//!
//! [`async`]: https://doc.rust-lang.org/std/keyword.async.html
//! [`await`]: https://doc.rust-lang.org/std/keyword.await.html
//

/* always compiled for internal use */

/* only compiled with the `future` feature */

/* re-exports */

#[cfg(feature = "future")]
mod reexports;

#[cfg(feature = "future")]
pub use all::*;
#[cfg(feature = "future")]
pub(crate) mod all {
    pub use super::reexports::*;
}
