// devela_base_core::sys::mem::reexports
//
//!
//

use crate::_reexport;

/* mem structs */

_reexport! { rust: core::mem,
    doc: "A wrapper to inhibit compiler from automatically calling `T`’s destructor.",
    ManuallyDrop
}
_reexport! { rust: core::mem,
    doc: "Opaque type representing the discriminant of an enum.",
    Discriminant
}

/* mem unions */

_reexport! { rust: core::mem,
    doc: "A wrapper type to construct uninitialized instances of `T`.",
    MaybeUninit
}
/* mem macros */

_reexport! { rust: core::mem,
    doc: "Expands to the offset in bytes of a field from the beginning of the given type.",
    offset_of
}

/* mem functions */

// NOTE: can't namespace this in `Mem`.
// NOTE: it shows a wrong deprecation message when re-exported. Ignore it.
_reexport! { rust: core::mem,
    doc: "Reinterprets the bits of a value of one type as another type.",
    transmute
}
