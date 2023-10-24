// devela::task::reexports::future
//
//! Reexported items.
//

use crate::meta::reexport;

reexport! { rust: core::future, local_module: "task",
    doc: "A future which never resolves, representing a computation that never finishes.",
    @Pending as FuturePending
}
reexport! { rust: core::future, local_module: "task",
    doc: "A Future that wraps a function returning [`TaskPoll`][crate::task::TaskPoll].",
    @PollFn as FuturePollFn
}
reexport! { rust: core::future, local_module: "task",
    doc: "A future that is immediately ready with a value.",
    @Ready as FutureReady
}
reexport! { rust: core::future, local_module: "task",
    doc: "An asynchronous computation obtained by use of
        [`async`](https://doc.rust-lang.org/std/keyword.async.html).",
    Future
}
reexport! { rust: core::future, local_module: "task",
    doc: "Conversion into a `Future`.",
    IntoFuture
}
reexport! { rust: core::future, local_module: "task",
    doc: "Creates a [`FutureReady`].",
    @ready as future_ready
}
reexport! { rust: core::future, local_module: "task",
    doc: "Creates a [`FuturePending`] future.",
    @pending as future_pending
}
