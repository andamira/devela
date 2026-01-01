// devela_base_core::work::future::coroutine::_reexport

use crate::_reexport;

_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    tag: crate::_TAG_RUNTIME!() crate::_TAG_CONCURRENCY!(),
    doc: "The trait implemented by builtin coroutine types.",
    Coroutine
}
_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    tag: crate::_TAG_RUNTIME!() crate::_TAG_CONCURRENCY!(),
    doc: "The result of a coroutine resumption.",
    CoroutineState
}
