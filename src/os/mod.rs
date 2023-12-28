// devela::os
//
//! OS-specific functionality, extends
//! `std::{`[`ffi`], [`os`]`}`.
//!
//! [`ffi`]: std::ffi
//! [`os`]: std::os
//

#[cfg(feature = "os_term")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "os_term")))]
pub mod term;

// re-export private sub-modules

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "os_term")]
pub use term::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "os_term")]
    pub use super::term::all::*;
}
