// devela::mem
//
//! Reexported items from `core`.
//

use crate::meta::reexport;

reexport! { rust: core::mem, local_module: "mem",
    doc: "A wrapper to inhibit compiler from automatically calling `T`’s destructor.",
    ManuallyDrop
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "A wrapper type to construct uninitialized instances of `T`.",
    MaybeUninit
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Opaque type representing the discriminant of an enum.",
    @Discriminant as EnumDiscriminant
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Returns a value uniquely identifying the enum variant in `v`.",
    @discriminant as enum_discriminant
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Returns the minimum alignment of the type of the value that `val` points to in bytes.",
    @align_of as mem_align_of
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Returns the align of the pointed-to value in bytes.",
    @align_of_val as mem_align_of_val
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Disposes of a value.",
    @drop as mem_drop
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Takes ownership and “forgets” about `t` *without running its destructor*.",
    @forget as mem_forget
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Returns true if dropping values of type T matters.",
    @needs_drop as mem_needs_drop
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Moves `src` into `dest`, returning the previous `dest` value.",
    @replace as mem_replace
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Swaps the values at two locations, without deinitializing either one.",
    @swap as mem_swap
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Replaces `dest` with `T::default()`, returning the previous `dest` value.",
    @take as mem_take
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Reinterprets the bits of a value of one type as another type.",
    @transmute as mem_transmute
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Reads `src` as having type `&Dst` without moving the contained value.",
    @transmute_copy as mem_transmute_copy
}
reexport! { rust: core::mem, local_module: "mem",
    doc: "Returns the value of type `T` represented by the all-zero byte-pattern.",
    @zeroed as mem_zeroed
}
