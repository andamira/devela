// devela::work::future::_reexport
//
//!
//
// WAIT: [future_join](https://github.com/rust-lang/rust/issues/91642)
// WAIT: [async_drop](https://github.com/rust-lang/rust/issues/126482)

use crate::{_reexport, _tags};

/* `core` future */

_reexport! { rust: core::future, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "A future which never resolves, representing a never finishing computation.",
    @Pending as FuturePending
}
_reexport! { rust: core::future, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "A Future that wraps a function returning [`AsyncPoll`].",
    @PollFn as FuturePollFn
}
_reexport! { rust: core::future, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "A future that is immediately ready with a value.",
    @Ready as FutureReady
}
_reexport! { rust: core::future, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "An asynchronous computation obtained by use of
        [`async`](https://doc.rust-lang.org/std/keyword.async.html).",
    Future
}
_reexport! { rust: core::future, location: "work/future", tag: _tags!(value concurrency runtime),
    doc: "Conversion into a `Future`.",
    IntoFuture
}

/* `core` task */

_reexport! { rust: core::task, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "The context of an asynchronous task.",
    @Context as AsyncContext
}
_reexport! { rust: core::task, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "Allows the implementor of a task executor to create a [`AsyncWaker`].",
    @RawWaker as AsyncRawWaker
}
_reexport! { rust: core::task, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "A virtual fn pointer table that specifies the behavior of a [`AsyncRawWaker`].",
    @RawWakerVTable as AsyncRawWakerVTable
}
_reexport! { rust: core::task, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "A handle for waking up a task by notifying its executor that it's ready to run.",
    @Waker as AsyncWaker
}
_reexport! { rust: core::task, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "Indicates whether a value is ready or if the current task is still pending.",
    @Poll as AsyncPoll
}
_reexport! { rust: core::task, location: "work/future", tag: _tags!(concurrency runtime),
    doc: "Extracts the successful type of a [`AsyncPoll<T>`].",
    @ready as async_ready
}
