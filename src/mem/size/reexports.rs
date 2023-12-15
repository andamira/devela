// devela::mem
//
//! Reexported items from `core`.
//

use crate::meta::reexport;

reexport! { rust: core::marker, local_module: "mem",
    doc: "Types with a constant size known at compile time.",
    Sized
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Returns the size of a type in bytes.",
    @size_of as mem_size_of
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Returns the size of the pointed-to value in bytes.",
    @size_of_val as mem_size_of_val
}
