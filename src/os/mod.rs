// devela::os
//
//! OS-specific, extends
//! `std::`[`os`][std::os].
//

/* contains always compiled items */

// ...

mod print;

/* feature-gated */

#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;

// re-export private sub-modules
#[doc(inline)]
pub use print::*;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "linux")]
pub use linux::all::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::print::*;

    #[doc(inline)]
    #[cfg(feature = "linux")]
    pub use super::linux::all::*;
}
