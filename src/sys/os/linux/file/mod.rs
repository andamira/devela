// devela::sys::os::linux::file
//
//! File-descriptor–centric ABI surface.
//!
//! This module groups Linux ABI items whose semantics are scoped to an
//! open file descriptor, including descriptor flags, control commands,
//! metadata queries, and extended control operations (`ioctl`).
//

mod fcntl; // LINUX_F_CMD
mod fd; // LINUX_[FILENO|O_FLAGS|SEEK]
mod ioctl; // LINUX_IOCTL
mod stat; // LinuxStat, LINUX_S_IFMT

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            fcntl::*,
            fd::*,
            ioctl::*,
            stat::*,
        };
    }
}
