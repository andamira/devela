// devela::os
//
//! OS-specific, extends [`std::os`].
//

pub(crate) mod all {
    #[cfg(feature = "linux")]
    #[doc(inline)]
    pub use super::linux::all::*;

    #[doc(inline)]
    pub use super::{print::*, term::*};
}

// use a feature instead of OS detection, so it can be used from `no_std`.
#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;
pub mod term;

mod print;

#[doc(inline)]
pub use {print::*, term::*};
