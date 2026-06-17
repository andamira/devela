// devela/src/sys/os/linux/io/file/fd/mod.rs
//
//! File-descriptor identity and positioning.
//

mod _raw; // Raw Linux file-descriptor constants.
mod fd; // LinuxFd
mod seek; // LinuxSeekFrom

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            fd::*,
            seek::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _raw::{LINUX_AT, LINUX_FILENO, LINUX_O_FLAGS, LINUX_SEEK},
        };
    }

}
