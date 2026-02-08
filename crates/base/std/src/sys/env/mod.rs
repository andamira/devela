// devela_base_std::sys::env
//
#![doc = crate::_DOC_SYS_ENV!()] // public
#![doc = crate::_doc!(modules: crate::sys; env)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: env)]
//

mod _reexport; // SYMLINK from /src/sys/env/_reexport_std.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
