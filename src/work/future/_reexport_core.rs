// devela/src/work/future/_reexport_core.rs
//
//!
//
// WAIT: [future_join](https://github.com/rust-lang/rust/issues/91642)
// WAIT: [async_drop](https://github.com/rust-lang/rust/issues/126482)

use crate::{_reexport, _tags};

/* `core` future */

_reexport! { rust: core::future,
    location: "work/future" => struct FuturePending, tag: _tags!(concurrency runtime),
    doc: "A future which never resolves, representing a never finishing computation.",
    @Pending as FuturePending
}
_reexport! { rust: core::future,
    location: "work/future" => struct FuturePollFn, tag: _tags!(concurrency runtime),
    doc: "A Future that wraps a function returning [`AsyncPoll`].",
    @PollFn as FuturePollFn
}
_reexport! { rust: core::future,
    location: "work/future" => struct FutureReady, tag: _tags!(concurrency runtime),
    doc: "A future that is immediately ready with a value.",
    @Ready as FutureReady
}
_reexport! { rust: core::future,
    location: "work/future" => trait Future, tag: _tags!(concurrency runtime),
    doc: "An asynchronous computation obtained by use of
        [`async`](https://doc.rust-lang.org/std/keyword.async.html).",
    Future
}
_reexport! { rust: core::future,
    location: "work/future" => trait IntoFuture, tag: _tags!(value concurrency runtime),
    doc: "Conversion into a `Future`.",
    IntoFuture
}

/* `core` task */

_reexport! { rust: core::task,
    location: "work/future" => struct AsyncContext, tag: _tags!(concurrency runtime),
    doc: "The context of an asynchronous task.",
    @Context as AsyncContext
}
_reexport! { rust: core::task,
    location: "work/future" => struct AsyncRawWaker, tag: _tags!(concurrency runtime),
    doc: "Allows the implementor of a task executor to create a [`AsyncWaker`].",
    @RawWaker as AsyncRawWaker
}
_reexport! { rust: core::task,
    location: "work/future" => struct AsyncRawWakerVTable, tag: _tags!(concurrency runtime),
    doc: "A virtual fn pointer table that specifies the behavior of a [`AsyncRawWaker`].",
    @RawWakerVTable as AsyncRawWakerVTable
}
_reexport! { rust: core::task,
    location: "work/future" => struct AsyncWaker, tag: _tags!(concurrency runtime),
    doc: "A handle for waking up a task by notifying its executor that it's ready to run.",
    @Waker as AsyncWaker
}
_reexport! { rust: core::task,
    location: "work/future" => enum AsyncPoll, tag: _tags!(concurrency runtime),
    doc: "Indicates whether a value is ready or if the current task is still pending.",
    @Poll as AsyncPoll
}
_reexport! { rust: core::task,
    location: "work/future" => macro async_ready, tag: _tags!(concurrency runtime),
    doc: "Extracts the successful type of a [`AsyncPoll<T>`].",
    @ready as async_ready
}
