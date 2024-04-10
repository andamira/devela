// devela::io
//
//! I/O functionality, extends `std::{`[`env`], [`fs`], [`io`], [`net`]`}`.
//!
//! [`env`]: mod@std::env
//! [`fs`]: std::fs
//! [`io`]: std::io
//! [`net`]: std::net
//! [`path`]: std::path
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

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
