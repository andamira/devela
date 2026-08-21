// devela/src/code/ops/call/_reexport_core.rs

use crate::{_reexport, _tags};

/* traits */

_reexport! { rust: core::ops,
    location: "code/ops/call" => trait Fn, tag: _tags!(code),
    doc: "The version of the call operator that takes an immutable receiver.", Fn
}
_reexport! { rust: core::ops,
    location: "code/ops/call" => trait FnMut, tag: _tags!(code),
    doc: "The version of the call operator that takes a mutable receiver.", FnMut
}
_reexport! { rust: core::ops,
    location: "code/ops/call" => trait FnOnce, tag: _tags!(code),
    doc: "The version of the call operator that takes a by-value receiver.", FnOnce
}
