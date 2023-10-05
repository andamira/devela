// devela::task::reexports
//
//! Reexported items.
//

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "The context of an asynchronous task.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*`::Context`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::Context as TaskContext;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Allows the implementor of a task executor to create a [`TaskWaker`].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*`RawWaker`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::RawWaker as TaskRawWaker;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A virtual function pointer table that specifies the behavior of a [`TaskRawWaker`].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*`::RawWakerVTable`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::RawWakerVTable as TaskRawWakerVTable;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A handle for waking up a task by notifying its executor that it is ready to be run.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*`Waker`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::Waker as TaskWaker;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
/// Indicates whether a value is available or if the current task has been scheduled
#[doc = "to receive a wakeup instead.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*`::Poll`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::Poll as TaskPoll;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Extracts the successful type of a [`TaskPoll<T>`].\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`task`](https://doc.rust-lang.org/core/task)*`::ready`.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub use core::task::ready as task_ready;

/// <span class="stab portability" title="re-exported from `alloc`">`alloc`</span>
#[doc = "The implementation of waking a task on an executor.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`alloc::`[`task`](https://doc.rust-lang.org/alloc/task)*`::Wake`.\n\n---"]
#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "task", feature = "alloc", target_has_atomic = "ptr")))
)]
pub use alloc::task::Wake as TaskWake;
