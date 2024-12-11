// devela::sys::os::linux::consts
//
//! Linux related constants.
//

pub(super) mod errno;
pub(super) mod fd;
pub(super) mod ioctl;
pub(super) mod signal;
pub(super) mod syscall;
pub(super) mod termios;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{
            errno::LINUX_ERRNO,
            fd::LINUX_FILENO,
            ioctl::LINUX_IOCTL,
            signal::{LINUX_SIGACTION, LINUX_SIGNAL},
            syscall::*,
            termios::{
                LINUX_TERMIOS_CFLAG, LINUX_TERMIOS_IFLAG, LINUX_TERMIOS_LFLAG, LINUX_TERMIOS_OFLAG,
            },
        };
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
