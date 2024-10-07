// devela::mem::ptr::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

// structs
reexport! { rust: core::ptr,
    doc: "`*mut T` but non-zero and *covariant*.",
    @NonNull as PtrNonNull
}

// macros
reexport! { rust: core::ptr,
    doc: "Create a `const` raw pointer to a place, without creating an intermediate reference.",
    @addr_of as ptr_addr_of
}
reexport! { rust: core::ptr,
    doc: "Create a `mut` raw pointer to a place, without creating an intermediate reference.",
    @addr_of_mut as ptr_addr_of_mut
}

// fns
reexport! { rust: core::ptr,
    doc: "Compares raw pointer addresses for equality, ignoring any metadata in fat pointers.",
    @addr_eq as ptr_addr_eq
}
reexport! { rust: core::ptr, // const unsafe
    doc: "Copies `count * size_of::<T>()` bytes from `src` to `dst`. Can overlap.",
    @copy as ptr_copy
}
reexport! { rust: core::ptr, // const unsafe
    doc: "Copies `count * size_of::<T>()` bytes from `src` to `dst`. Must *not* overlap.",
    @copy_nonoverlapping as ptr_copy_nonverlapping
}
reexport! { rust: core::ptr, // unsafe
    doc: "Executes the destructor (if any) of the pointed-to value.",
    @drop_in_place as ptr_drop_in_place
}
reexport! { rust: core::ptr,
    doc: "Compares raw pointers for equality.",
    @eq as ptr_eq
}
reexport! { rust: core::ptr, // const
    doc: "Convert an exclusive reference to a raw pointer.",
    @from_mut as ptr_from_mut
}
reexport! { rust: core::ptr, // const
    doc: "Convert a shared reference to a raw pointer.",
    @from_ref as ptr_from_ref
}
reexport! { rust: core::ptr,
    doc: "Hash a raw pointer.",
    @hash as ptr_hash
}
reexport! { rust: core::ptr, // const
    doc: "Creates a null raw pointer.",
    @null as ptr_null
}
reexport! { rust: core::ptr, // const
    doc: "Creates a null mutable raw pointer.",
    @null_mut as ptr_null_mut
}
reexport! { rust: core::ptr, // unsafe
    doc: "Reads the value from src without moving it.",
    @read as ptr_read
}
reexport! { rust: core::ptr, // unsafe
    doc: "Reads the value from src without moving it.",
    @read_unaligned as ptr_read_unaligned
}
reexport! { rust: core::ptr, // unsafe
    doc: "Performs a volatile read of the value from src without moving it.",
    @read_volatile as ptr_read_volatile
}
reexport! { rust: core::ptr, // unsafe
    doc: "Moves src into the pointed dst, returning the previous dst value.",
    @replace as ptr_replace
}
reexport! { rust: core::ptr,
    doc: "Forms a raw slice from a pointer and a length.",
    @slice_from_raw_parts as ptr_slice_from_raw_parts
}
reexport! { rust: core::ptr,
    doc: "Forms a mutable raw slice from a mutable pointer and a length.",
    @slice_from_raw_parts_mut as ptr_slice_from_raw_parts_mut
}
reexport! { rust: core::ptr, // unsafe
    doc: "Swaps the values at two mutable locations of the same type, without deinitializing.",
    @swap as ptr_swap
}
reexport! { rust: core::ptr, // unsafe
    doc: "Swaps the two regions of memory beginning at `x` and `y`. Must *not* overlap.",
    @swap_nonoverlapping as ptr_swap_nonoverlapping
}
reexport! { rust: core::ptr, // unsafe
    doc: "Overwrites a memory location with `src` without reading or dropping.",
    @write as ptr_write
}
reexport! { rust: core::ptr, // unsafe
    doc: "Sets `count * size_of::<T>()` bytes of memory starting at `dst` to `val`.",
    @write_bytes as ptr_write_bytes
}
reexport! { rust: core::ptr, // unsafe
    doc: "Overwrites a memory location with `src` without reading or dropping.",
    @write_unaligned as ptr_write_unaligned
}
reexport! { rust: core::ptr, // unsafe
    doc: "Performs a volatile write of a memory location with `src` without reading or dropping.",
    @write_volatile as ptr_write_volatile
}
