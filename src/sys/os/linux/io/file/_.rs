// devela/src/sys/os/linux/io/file/_.rs
//
//! File-descriptor–centric ABI surface.
//!
//! This module groups Linux ABI items whose semantics are scoped to an
//! open file descriptor, including descriptor flags, control commands,
//! metadata queries, and extended control operations (`ioctl`).
//

crate::mods_in! {
    mod fcntl; // LINUX_F_CMD
    mod_ fd; // LinuxFd, LinuxSeekFrom, (LINUX_<AT|FILENO|O_FLAGS|SEEK>)
    mod ioctl; // LINUX_IOCTL
    mod open; // LinuxOpenOptions
    mod pipe; // LinuxPipe, LinuxPipeFlags
    mod_ stat; // LinuxStat, LinuxFileType, (LINUX_S_IFMT)
}
crate::mods_out! { // _mods, crate_internals
    _mods {
        pub use super::{
            fcntl::*,
            fd::_all::*,
            ioctl::*,
            open::*,
            pipe::*,
            stat::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            fd::_crate_internals::*,
            stat::_crate_internals::*,
        };
    }
}
