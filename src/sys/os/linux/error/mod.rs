// devela/src/sys/os/linux/error/mod.rs
//
//! Linux error handling ABI and typed representations.
//

mod consts; // LINUX_ERRNO, LINUX_EXIT
mod error; // LinuxError, LinuxResult

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            consts::*,
            error::*,
        };
    }
}
