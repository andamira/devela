// devela::os::linux::consts
//
//! Linux related constants.
//

mod errno;
mod fd;
mod ioctl;
mod signal;
mod syscall;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        errno::LINUX_ERRNO,
        fd::LINUX_FILENO,
        ioctl::LINUX_IOCTL,
        signal::{LINUX_SIGACTION, LINUX_SIGNAL},
        syscall::*,
    };
}
