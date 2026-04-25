// devela::work::task::coro::_reexport_core

#[allow(unused_imports, reason = "nightly_coro feature-gate")]
use crate::{_reexport, _tags};

_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    location: "work/future",
    tag: _tags!(concurrency runtime),
    doc: "The trait implemented by builtin coroutine types.",
    Coroutine
}
_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    location: "work/future",
    tag: _tags!(concurrency runtime result),
    doc: "The result of a coroutine resumption.",
    CoroutineState
}
