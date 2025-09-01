// devela::sys::mem::ptr
//
#![doc = crate::_DOC_SYS_MEM_PTR!()]
//!
// #![doc = crate::_doc!(extends: ptr)]
//

crate::mod_path!(_c "../../../../libs/base/src/sys/mem/ptr/reexports.rs");

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod fat;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use devela_base::sys::mem::Ptr;

        pub use super::_c::*;
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::fat::FatPtr;
    }
    _always {
        pub use super::_c::*;
    }
}
