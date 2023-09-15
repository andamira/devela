// devela::os::linux::consts
//
//! Linux constants.
//

mod errno;
mod fd;
mod ioctl;
mod syscall;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{errno::*, fd::*, ioctl::*, syscall::*};
}
