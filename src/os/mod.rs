// devela::os
//
//! OS-specific functionality, extends
//! `std::{`[`arch`], [`ffi`], [`os`]`}`.
//!
//! [`arch`]: std::arch
//! [`ffi`]: std::ffi
//! [`os`]: std::os
//

pub mod arch;
pub mod ffi;

#[cfg(feature = "os_term")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "os_term")))]
pub mod term;

// re-export public sub-modules

#[doc(no_inline)]
pub use {arch::all::*, ffi::all::*};

#[doc(no_inline)]
#[cfg(feature = "os_term")]
pub use term::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "os_term")]
    pub use super::term::all::*;
}
