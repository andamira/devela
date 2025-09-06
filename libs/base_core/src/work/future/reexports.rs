// devela_base_core::work::future::reexports
//
//!
//
// WAIT: [future_join](https://github.com/rust-lang/rust/issues/91642)
// WAIT: [async_drop](https://github.com/rust-lang/rust/issues/126482)

use crate::_reexport;

/* `core` future */

_reexport! { rust: core::future,
    doc: "A future which never resolves, representing a computation that never finishes.",
    @Pending as FuturePending
}
_reexport! { rust: core::future,
    doc: "A Future that wraps a function returning [`TaskPoll`].",
    @PollFn as FuturePollFn
}
_reexport! { rust: core::future,
    doc: "A future that is immediately ready with a value.",
    @Ready as FutureReady
}
_reexport! { rust: core::future,
    doc: "An asynchronous computation obtained by use of
        [`async`](https://doc.rust-lang.org/std/keyword.async.html).",
    Future
}
_reexport! { rust: core::future,
    doc: "Conversion into a `Future`.",
    IntoFuture
}

/* `core` task */

_reexport! { rust: core::task,
    doc: "The context of an asynchronous task.",
    @Context as TaskContext
}
_reexport! { rust: core::task,
    doc: "Allows the implementor of a task executor to create a [`TaskWaker`].",
    @RawWaker as TaskRawWaker
}
_reexport! { rust: core::task,
    doc: "A virtual function pointer table that specifies the behavior of a [`TaskRawWaker`].",
    @RawWakerVTable as TaskRawWakerVTable
}
_reexport! { rust: core::task,
    doc: "A handle for waking up a task by notifying its executor that it is ready to be run.",
    @Waker as TaskWaker
}
_reexport! { rust: core::task,
    doc: "Indicates whether a value is ready or if the current task is still pending.",
    @Poll as TaskPoll
}
_reexport! { rust: core::task,
    doc: "Extracts the successful type of a [`TaskPoll<T>`].",
    @ready as task_ready
}
