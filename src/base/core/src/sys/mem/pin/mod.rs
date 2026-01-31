// devela_base_core::sys::mem::pin
//
#![doc = crate::_DOC_SYS_MEM_PIN!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; pin)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: pin)]
//

mod _reexport; // SYMLINK from /src/sys/mem/pin/_reexport_core.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
