// devela_base_std::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; fd)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: os)]
//

mod _reexport; // SYMLINK from /src/sys/os/fd/_reexport_std.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
