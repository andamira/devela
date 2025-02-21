// devela::work::future::reexports
//
//! Reexported items.
//
// WAIT: [future_join](https://github.com/rust-lang/rust/issues/91642)
// WAIT: [async_drop](https://github.com/rust-lang/rust/issues/126482)

use crate::code::reexport;

/* from either `alloc` or `portable-atomic-util` and `alloc` */

#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub use crate::_dep::portable_atomic_util::task::Wake as TaskWake;
//
#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub use crate::_dep::_alloc::task::Wake as TaskWake;

/* `core` future */

reexport! { rust: core::future,
    doc: "A future which never resolves, representing a computation that never finishes.",
    @Pending as FuturePending
}
reexport! { rust: core::future,
    doc: "A Future that wraps a function returning [`TaskPoll`].",
    @PollFn as FuturePollFn
}
reexport! { rust: core::future,
    doc: "A future that is immediately ready with a value.",
    @Ready as FutureReady
}
reexport! { rust: core::future,
    doc: "An asynchronous computation obtained by use of
        [`async`](https://doc.rust-lang.org/std/keyword.async.html).",
    Future
}
reexport! { rust: core::future,
    doc: "Conversion into a `Future`.",
    IntoFuture
}

/* `core` task */

reexport! { rust: core::task,
    doc: "The context of an asynchronous task.",
    @Context as TaskContext
}
reexport! { rust: core::task,
    doc: "Allows the implementor of a task executor to create a [`TaskWaker`].",
    @RawWaker as TaskRawWaker
}
reexport! { rust: core::task,
    doc: "A virtual function pointer table that specifies the behavior of a [`TaskRawWaker`].",
    @RawWakerVTable as TaskRawWakerVTable
}
reexport! { rust: core::task,
    doc: "A handle for waking up a task by notifying its executor that it is ready to be run.",
    @Waker as TaskWaker
}
reexport! { rust: core::task,
    doc: "Indicates whether a value is ready or if the current task is still pending.",
    @Poll as TaskPoll
}
reexport! { rust: core::task,
    doc: "Extracts the successful type of a [`TaskPoll<T>`].",
    @ready as task_ready
}
