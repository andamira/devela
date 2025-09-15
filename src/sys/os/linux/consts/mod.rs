// devela::sys::os::linux::consts
//
//! Linux related constants.
//

pub(super) mod clock; // LINUX_CLOCK
pub(super) mod errno; // LINUX_[ERRNO|EXIT].
pub(super) mod file; // LINUX_[FD|O_FLAGS].
pub(super) mod ioctl; // LINUX_IOCTL.
pub(super) mod signal; // LINUX_[SIGACTION|SIGNAL].
pub(super) mod termios; // LINUX_TERMIOS_[I|O|C|L]FLAG.

crate::structural_mods! { // _mods
    _mods {
        pub use super::{clock::*, errno::*, file::*, ioctl::*, signal::*, termios::*};
    }
}
