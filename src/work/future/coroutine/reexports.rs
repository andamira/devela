// devela::work::future::coroutine::reexports
//
//! Reexported items.
//

use crate::_reexport;

_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    doc: "The trait implemented by builtin coroutine types.",
    Coroutine
}
_reexport! { rust: core::ops,
    extra_flags:(nightly_coro),
    doc: "The result of a coroutine resumption.",
    CoroutineState
}
