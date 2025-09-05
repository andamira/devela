// devela::sys::mem::ptr
//
#![doc = crate::_DOC_SYS_MEM_PTR!()]
//!
// #![doc = crate::_doc!(extends: ptr)]
//

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod fat;

// re-exports
crate::mod_path!(_c "../../../../libs/base/src/sys/mem/ptr/reexports.rs");

crate::structural_mods! { // _mods, _always
    _mods {
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::fat::FatPtr;

        // re-exports
        pub use super::_c::*;
        #[doc(inline)]
        pub use devela_base::sys::mem::Ptr;
    }
}
