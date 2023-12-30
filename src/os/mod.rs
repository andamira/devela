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

// re-export public sub-modules

#[doc(no_inline)]
#[allow(unused)]
pub use {arch::all::*, ffi::all::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{arch::all::*, ffi::all::*};
}
