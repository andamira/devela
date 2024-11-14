// devela::sys::io
//
//! I/O functionality.
//!
#![doc = crate::doc_!(extends: io)]
//!
//! # Derived work
#![doc = include_str!("./define_no_std_io/MODIFICATIONS.md")]
//

// warnings:
#![allow(unused_imports)]

#[cfg(not(feature = "std"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "sys")))]
mod define_no_std_io;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "sys")))]
mod reexport_std;

#[cfg(not(feature = "std"))]
pub use define_no_std_io::*;
#[cfg(feature = "std")]
pub use reexport_std::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(not(feature = "std"))]
    pub use super::define_no_std_io::*;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::reexport_std::*;
}
