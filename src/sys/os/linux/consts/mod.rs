// devela::sys::os::linux::consts
//
//! Linux related constants.
//

pub(super) mod errno;
pub(super) mod fd;
pub(super) mod ioctl;
pub(super) mod signal;
pub(super) mod termios;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{errno::*, fd::*, ioctl::*, signal::*, termios::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
