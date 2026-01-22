// devela_base_std::sys::env
//
#![doc = crate::_DOC_SYS_ENV!()]
//

mod _reexport; // SYMLINK from /src/sys/env/_reexport_std.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
