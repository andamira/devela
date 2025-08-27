// devela::sys::mem::ptr
//
#![doc = crate::_DOC_SYS_MEM_PTR!()]
//!
// #![doc = crate::_doc!(extends: ptr)]
//

mod namespace; // Ptr
mod reexports; // ::core::ptr::*

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod fat;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{namespace::Ptr, reexports::*};
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::fat::FatPtr;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
