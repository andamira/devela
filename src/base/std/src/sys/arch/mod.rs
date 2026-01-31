// devela_base_std::sys::arch
//
#![doc = crate::_DOC_SYS_ARCH!()] // public
#![doc = crate::_doc!(modules: crate::sys; arch)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: arch)]
//

mod _reexport; // SYMLINK from /src/sys/arch/_reexport_std.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
