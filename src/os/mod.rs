// devela::os
//
//! OS-specific functionality, extends
//! `std::{`[`arch`], [`ffi`], [`os`]`}`.
//!
//! [`arch`]: std::arch
//! [`ffi`]: std::ffi
//! [`os`]: std::os
//

// warnings
#![cfg_attr(not(feature = "os"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_os", forbid(unsafe_code))]

/* modules */

// always compiled, public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "os")))]
pub mod arch;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "os")))]
pub mod ffi;

/* re-exports */

// always compiled, public
#[doc(no_inline)]
pub use {arch::all::*, ffi::all::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{arch::all::*, ffi::all::*};
}
