// devela_base_std::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()]
//

crate::structural_mods! { //_mods
    _mods {
        // re-exports
        #[doc = crate::_TAG_FS!()]
        #[cfg(any(unix, target_os = "wasi", doc))]
        #[cfg(not(miri))]
        pub use ::std::os::fd::{
            OwnedFd as FdOwned,
            AsFd, AsRawFd as AsFdRaw,
            FromRawFd as FromFdRaw,
            IntoRawFd as IntoFdRaw,
        };
        #[doc = crate::_TAG_FS!()]
        #[doc = crate::_TAG_LIFETIME!()]
        #[cfg(any(unix, target_os = "wasi", doc))]
        #[cfg(not(miri))]
        pub use ::std::os::fd::{
            BorrowedFd as FdBorrowe
        };
    }
}
