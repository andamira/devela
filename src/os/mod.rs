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

// do not block for non-linux oses so it can be used from no_std.
#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;
pub mod term;

#[doc(inline)]
pub use term::*;

mod print;
pub use print::*;
