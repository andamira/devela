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
    @addr_of as addr_of
}
reexport! { rust: core::ptr,
    doc: "Create a `mut` raw pointer to a place, without creating an intermediate reference.",
    @addr_of_mut as addr_of_mut
}
