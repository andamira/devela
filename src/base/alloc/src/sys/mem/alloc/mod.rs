// devela_base_alloc::sys::mem::alloc
//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; alloc)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc)]
//

mod _reexport; // SYMLINK from /src/sys/mem/alloc/_reexport_alloc.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
