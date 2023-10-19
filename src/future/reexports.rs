// devela::future::reexports
//
//! Reexported items.
//

use crate::codegen::reexport;

reexport! { rust: core::future, local_module: "future",
    doc: "A future which never resolves, representing a computation that never finishes.",
    @Pending as FuturePending
}
reexport! { rust: core::future, local_module: "future",
    doc: "A Future that wraps a function returning [`TaskPoll`][crate::task::TaskPoll].",
    @PollFn as FuturePollFn
}
reexport! { rust: core::future, local_module: "future",
    doc: "A future that is immediately ready with a value.",
    @Ready as FutureReady
}
reexport! { rust: core::future, local_module: "future",
    doc: "An asynchronous computation obtained by use of
        [`async`](https://doc.rust-lang.org/std/keyword.async.html).",
    Future
}
reexport! { rust: core::future, local_module: "future",
    doc: "Conversion into a `Future`.",
    IntoFuture
}
reexport! { rust: core::future, local_module: "future",
    doc: "Creates a [`FutureReady`].",
    @ready as future_ready
}
reexport! { rust: core::future, local_module: "future",
    doc: "Creates a [`FuturePending`] future.",
    @pending as future_pending
}
