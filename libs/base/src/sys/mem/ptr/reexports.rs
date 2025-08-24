// devela_base::sys::mem::ptr::reexports
//
//!
//

use crate::_reexport;

// structs
_reexport! { rust: core::ptr,
    doc: "`*mut T` but non-zero and *covariant*.",
    @NonNull as PtrNonNull
}

// functions
_reexport! { rust: core::ptr,
    doc: "Compares the addresses of the two function pointers for equality.",
    fn_addr_eq // NOTE: Can't be namespaced yet in Ptr::fn_addr_eq because of WAIT:fn_ptr_trait
}
