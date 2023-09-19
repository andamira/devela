// devela::os::linux::consts
//
//! Linux related constants.
//

mod errno;
mod fd;
mod ioctl;
mod signal;
mod syscall;
mod termios;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
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
