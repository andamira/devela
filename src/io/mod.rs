// devela::io
//
//! I/O functionality, extends
//! `std::{`[`env`], [`fs`], [`io`], [`net`], [`path`]`}`.
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
#![cfg_attr(feature = "safe_io", forbid(unsafe_code))]

/* feature-gated, non-public modules */

#[cfg(feature = "io")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
mod path;

#[cfg(not(feature = "std"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
mod define_no_std_io;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
mod reexport_std;

#[cfg(feature = "io")]
pub use path::all::*;

#[cfg(not(feature = "std"))]
pub use define_no_std_io::*;
#[cfg(feature = "std")]
pub use reexport_std::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "io")]
    pub use super::path::all::*;
    //
    #[doc(inline)]
    #[cfg(not(feature = "std"))]
    pub use super::define_no_std_io::*;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::reexport_std::*;
}
