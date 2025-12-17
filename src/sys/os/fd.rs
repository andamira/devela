// devela::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()]
//

crate::structural_mods! { //_mods
    _mods {
        // re-exports
        pub use devela_base_core::sys::os::fd::{
            FdRaw,
        };
        #[cfg(feature = "std")]
        pub use devela_base_std::sys::os::fd::*;
    }
}
