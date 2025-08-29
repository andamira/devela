// devela::sys::os::linux::consts
//
//! Linux related constants.
//

pub(super) mod errno; // LINUX_[ERRNO|EXIT].
pub(super) mod file; // LINUX_[FD|O_FLAGS].
pub(super) mod ioctl; // LINUX_IOCTL.
pub(super) mod signal; // LINUX_[SIGACTION|SIGNAL].
pub(super) mod termios; // LINUX_TERMIOS_[I|O|C|L]FLAG.

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused_imports)]
        pub use super::{errno::*, file::*, ioctl::*, signal::*, termios::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
