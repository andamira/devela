// devela::future
//
//! Reexported items re-exported from `core`.
//

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
/// Creates a future which never resolves, representing a computation that never finishes.
#[doc = "\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::Pending;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A Future that wraps a function returning [`Poll`][crate::task::Poll].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::PollFn;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A future that is immediately ready with a value.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::Ready;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "An asynchronous computation obtained by use of [`async`].\n\n"]
/// [`async`]: https://doc.rust-lang.org/std/keyword.async.html
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::Future;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Conversion into a `Future`.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::IntoFuture;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Creates a future that is immediately ready with a value.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::ready as future_ready;
