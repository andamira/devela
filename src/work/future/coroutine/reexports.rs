// devela::work::future::coroutine::reexports
//
//! Reexported items.
//

use crate::reexport;

reexport! { rust: core::ops,
    extra_features: "nightly_coro",
    doc: "The trait implemented by builtin coroutine types.",
    Coroutine
}
reexport! { rust: core::ops,
    extra_features: "nightly_coro",
    doc: "The result of a coroutine resumption.",
    CoroutineState
}
