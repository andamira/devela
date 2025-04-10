// devela::sys::mem::ptr::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

// macros
reexport! { rust: core::ptr,
    doc: "Create a `const` raw pointer to a place, without creating an intermediate reference.",
    @addr_of as addr_of
}
reexport! { rust: core::ptr,
    doc: "Create a `mut` raw pointer to a place, without creating an intermediate reference.",
    @addr_of_mut as addr_of_mut
}

// structs
reexport! { rust: core::ptr,
    doc: "`*mut T` but non-zero and *covariant*.",
    @NonNull as PtrNonNull
}

// functions
reexport! { rust: core::ptr,
    doc: "Compares the addresses of the two function pointers for equality.",
    fn_addr_eq // NOTE: Can't be namespaced yet in Ptr::fn_addr_eq because of WAIT:fn_ptr_trait
}
