// devela_base_core::sys::mem::borrow
//
#![doc = crate::_DOC_SYS_MEM_BORROW!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; ptr)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: borrow)]
//

mod _reexport; // SYMLINK from /src/sys/mem/borrow/_reexport_core.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
