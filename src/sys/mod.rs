// devela::sys
//
//! System interfaces and hardware abstractions, <small>extends
//! `std::{`[`arch`], [`env`], [`ffi`], [`fs`], [`io`], [`net`], [`os`],
//! [`path`], [`simd`]`}`.</small>
//!
//! [`arch`]: std::arch
//! [`env`]: mod@std::env
//! [`ffi`]: std::ffi
//! [`fs`]: std::fs
//! [`io`]: std::io
//! [`net`]: std::net
//! [`os`]: std::os
//! [`path`]: std::path
//! [`simd`]: std::simd
//

// safety:
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

/* always compiled */

mod arch;
mod ffi;
mod io;

pub use {arch::*, ffi::*, io::*};

/* feature-gated */

#[cfg(feature = "sys")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "sys")))]
mod path;

#[cfg(feature = "sys")]
pub use path::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{arch::all::*, ffi::all::*, io::all::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "sys")]
    pub use super::path::all::*;
}
