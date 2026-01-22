// devela_base_core::sys::mem::ptr
//
#![doc = crate::_DOC_SYS_MEM_PTR!()]
//

mod _reexport; // SYMLINK from /src/sys/mem/ptr/_reexport_core.rs
mod namespace; // Ptr

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::Ptr;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
