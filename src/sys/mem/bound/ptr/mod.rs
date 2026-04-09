// devela::sys::mem::bound::ptr
//
#![doc = crate::_DOC_SYS_MEM_BOUND_PTR!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; ptr)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: ptr)]
//

mod _reexport_core;

mod namespace; // Ptr

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod fat;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::Ptr;

        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
        pub use super::fat::FatPtr;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
