// devela::os::linux::consts
//
//! Linux constants.
//

mod errno;
mod fd;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{errno::*, fd::*};
}
