// devela_base_core::sys::mem::ptr::_reexport

use crate::{_TAG_MEM, _TAG_NICHE, _reexport};

// structs
_reexport! { rust: core::ptr, location: "sys/mem", tag: _TAG_MEM!() _TAG_NICHE!(),
    doc: "`*mut T` but non-zero and *covariant*.",
    @NonNull as PtrNonNull
}

// functions
_reexport! { rust: core::ptr, location: "sys/mem", tag: _TAG_MEM!(),
    doc: "Compares the addresses of the two function pointers for equality.",
    fn_addr_eq // NOTE: Can't be namespaced yet in Ptr::fn_addr_eq because of WAIT:fn_ptr_trait
}
