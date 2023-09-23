// devela::os
//
//! OS-specific, extends [`std::os`].
//

pub(crate) mod all {
    #[cfg(feature = "linux")]
    #[doc(inline)]
    pub use super::linux::all::*;

    #[doc(inline)]
    pub use super::{print::*, terminal::*};
}

// do not block for non-linux oses so it can be used from no_std.
#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;
pub mod terminal;

#[doc(inline)]
pub use terminal::*;

mod print;
pub use print::*;
