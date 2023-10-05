// devela::future::reexports
//
//! Reexported items.
//

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
/// A future which never resolves, representing a computation that never finishes.
#[doc = "\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*`::Pending`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::Pending as FuturePending;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A Future that wraps a function returning [`TaskPoll`][crate::task::TaskPoll].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*`PollFn`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::PollFn as FuturePollFn;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A future that is immediately ready with a value.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*`::Ready`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::Ready as FutureReady;

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
#[doc = "Creates a [`FutureReady`].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*`::ready`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::ready as future_ready;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Creates a [`FuturePending`] future.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`future`](https://doc.rust-lang.org/core/future)*`::pending`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub use core::future::pending as future_pending;
