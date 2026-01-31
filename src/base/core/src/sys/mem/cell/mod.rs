// devela_base_core::sys::mem::cell
//
#![doc = crate::_DOC_SYS_MEM_CELL!()] // public
#![doc = crate::_doc!(modules: crate::sys::mem; cell)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: cell)]
//

mod _reexport; // SYMLINK from /src/sys/mem/cell/_reexport_core.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
