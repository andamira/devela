// devela::code::any::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { rust: core::any,
    doc: "A trait to emulate dynamic typing.",
    Any
}
reexport! { rust: core::any,
    doc: "Represents a globally unique identifier for a type.",
    TypeId
}
