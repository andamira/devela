// devela_base_std::sys::os
//
//! Unix-like file descriptors.
//

crate::structural_mods! { //_mods
    _mods {
        // re-exports
        pub use ::std::os::fd::{
            BorrowedFd as FdBorrowed, OwnedFd as FdOwned,
            AsFd,
            AsRawFd as AsFdRaw,
            FromRawFd as FromFdRaw,
            IntoRawFd as IntFdRaw,
        };
    }
}
