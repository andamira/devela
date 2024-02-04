// devela::work::async::coroutine::reexports
//
//! Reexported items.
//

use crate::code::reexport;

reexport! { rust: core::ops, local_module: "work",
    extra_features: "nightly_coro",
    doc: "The trait implemented by builtin coroutine types.",
    Coroutine
}
reexport! { rust: core::ops, local_module: "work",
    extra_features: "nightly_coro",
    doc: "The result of a coroutine resumption.",
    CoroutineState
}
