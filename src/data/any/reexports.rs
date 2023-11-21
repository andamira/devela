// devela::any::reexports
//
//! Reexported items from `core`.
//

use crate::meta::reexport;

reexport! { rust: core::any, local_module: "any",
    doc: "A trait to emulate dynamic typing.",
    Any
}
reexport! { rust: core::any, local_module: "any",
    doc: "Represents a globally unique identifier for a type.",
    TypeId
}
