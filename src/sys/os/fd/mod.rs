// devela::sys::os::fd
//
//! Unix-like file descriptors.
//

crate::structural_mods! { //_mods
    _mods {
        // re-exports
        pub use devela_base_core::sys::os::fd::{
            FdRaw,
        };
        #[cfg(feature = "std")]
        pub use devela_base_std::sys::os::fd::{
            AsFd, AsFdRaw, FromFdRaw, IntoFdRaw,
        };
    }
}
