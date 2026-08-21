// devela/src/code/ops/_reexport_core.rs

use crate::{_reexport, _tags};

/* enums */

_reexport! { rust: core::ops,
    location: "code/ops" => enum ControlFlow, tag: _tags!(code),
    doc: "Used to tell an operation whether it should exit early or go on as usual.", ControlFlow
}

/* `core::clone` */

// NOTE: the trait and the derive macro have the same name
_reexport! { rust: core::clone,
    location: "code/ops" /* … */, tag: _tags!(value),
    doc: "A common trait that allows explicit creation of a duplicate value. (Derivable)", Clone
}
