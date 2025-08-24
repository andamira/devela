// devela_base::code::any::reexports
//
//!
//

use crate::_reexport;

_reexport! { rust: core::any,
    doc: "A trait to emulate dynamic typing.",
    Any
}
_reexport! { rust: core::any,
    doc: "Represents a globally unique identifier for a type.",
    TypeId
}
_reexport! { rust: core::any,
    doc: "Returns the name of a type as a string slice.",
    @type_name as any_type_name
}
