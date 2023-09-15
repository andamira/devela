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
        errno::ERRNO,
        fd::FILENO,
        ioctl::IOCTL,
        signal::{SYS_SIGACTION, SYS_SIGNAL},
        syscall::{SYS_AARCH64, SYS_ARM, SYS_RISCV, SYS_X86, SYS_X86_64},
    };
}
