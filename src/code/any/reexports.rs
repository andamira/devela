// devela::code::any::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::any,
    doc: "A trait to emulate dynamic typing.",
    Any
}
reexport! { rust: core::any,
    doc: "Represents a globally unique identifier for a type.",
    TypeId
}
reexport! { rust: core::any,
    doc: "Returns the name of a type as a string slice.",
    @type_name as any_type_name
}
