// devela::exec::async::coroutine::reexports
//
//! Reexported items.
//

use crate::code::reexport;

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
