// devela_base_core::sys::mem::bound::ptr
//
#![doc = crate::_DOC_SYS_MEM_BOUND_PTR!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; ptr)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: ptr)]
//

mod _reexport; // SYMLINK from /src/sys/mem/bound/ptr/_reexport_core.rs
mod namespace; // Ptr

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::Ptr;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
