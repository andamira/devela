// devela::sys::mem::bound::ptr::_reexport_core

use crate::{_reexport, _tags};

// structs
_reexport! { rust: core::ptr, location: "sys/mem", tag: _tags!(mem niche),
    doc: "`*mut T` but non-zero and *covariant*.",
    @NonNull as PtrNonNull
}

// functions
_reexport! { rust: core::ptr, location: "sys/mem", tag: _tags!(mem),
    doc: "Compares the addresses of the two function pointers for equality.",
    fn_addr_eq // NOTE: Can't be namespaced yet in Ptr::fn_addr_eq because of WAIT:fn_ptr_trait
}
