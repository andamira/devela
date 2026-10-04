//
//! File-descriptor identity and positioning.
//

crate::mods_in! {
    mod _raw; // Raw Linux file-descriptor constants.

    #[crate::macro_apply(crate::_linux_syscall)]
    mod fd; // LinuxFd
    mod seek; // LinuxSeekFrom
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        #[crate::macro_apply(crate::_linux_syscall)]
        pub use super::fd::*;
        pub use super::seek::*;
    }
    _crate_internals {
        pub(crate) use super::{
            _raw::{LINUX_AT, LINUX_FILENO, LINUX_O_FLAGS, LINUX_SEEK},
        };
    }
}
