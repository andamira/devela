// devela::os::linux::consts
//
//! Linux constants.
//

pub mod errno;
pub mod fd;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{errno::*, fd::*};
}
