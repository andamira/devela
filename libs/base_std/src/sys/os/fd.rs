// devela_base_std::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()]
//

crate::structural_mods! { //_mods
    _mods {
        // re-exports
        #[cfg(any(
            unix,
            target_os = "hermit",
            target_os = "trusty",
            target_os = "wasi",
            target_os = "motor",
            doc
        ))]
        pub use ::std::os::fd::{
            BorrowedFd as FdBorrowed, OwnedFd as FdOwned,
            AsFd,
            AsRawFd as AsFdRaw,
            FromRawFd as FromFdRaw,
            IntoRawFd as IntoFdRaw,
        };
    }
}
