// devela::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; fd)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: os)]
//

#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/sys/os/fd/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;

        pub use devela_base_core::sys::os::fd::{
            FdRaw,
        };
    }
}
