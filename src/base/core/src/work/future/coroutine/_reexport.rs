// devela_base_core::work::future::coroutine::_reexport

#[allow(unused_imports, reason = "nightly_coro feature-gate")]
use crate::{_TAG_CONCURRENCY, _TAG_RESULT, _TAG_RUNTIME, _reexport};

_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    location: "work/future",
    tag: _TAG_CONCURRENCY!() _TAG_RUNTIME!(),
    doc: "The trait implemented by builtin coroutine types.",
    Coroutine
}
_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    location: "work/future",
    tag: _TAG_CONCURRENCY!() _TAG_RUNTIME!() _TAG_RESULT!(),
    doc: "The result of a coroutine resumption.",
    CoroutineState
}
