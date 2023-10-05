// devela::task
//
//! Reexported items re-exported from `core`.
//

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "The context of an asynchronous task.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::Context;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Allows the implementor of a task executor to create a [`Waker`].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::RawWaker;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A virtual function pointer table that specifies the behavior of a [`RawWaker`].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::RawWakerVTable;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A handle for waking up a task by notifying its executor that it is ready to be run.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::Waker;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
/// Indicates whether a value is available or if the current task has been scheduled
#[doc = "to receive a wakeup instead.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::Poll;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Extracts the successful type of a [`Poll<T>`].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::ready as task_ready;

/// <span class="stab portability" title="re-exported from `alloc`">`core`</span>
#[doc = "The implementation of waking a task on an executor.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`alloc::`[`task`](https://doc.rust-lang.org/alloc/task)*.\n\n---"]
#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "alloc", target_has_atomic = "ptr")))
)]
pub use alloc::task::Wake;
