// devela::sys
//
//! System interfaces and hardware abstractions, <br/><small>extends
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

mod arch;
mod env;
mod ffi;
mod io;
#[allow(unused_imports)]
pub use {arch::*, env::*, ffi::*, io::*};

#[cfg(feature = "log")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "log")))]
mod log;
#[cfg(feature = "log")]
pub use log::*;

#[cfg(feature = "sys")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "sys")))]
mod path;
#[allow(unused_imports)]
#[cfg(feature = "sys")]
pub use path::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{arch::all::*, env::all::*, ffi::all::*, io::all::*};

    #[doc(inline)]
    #[cfg(feature = "log")]
    #[allow(unused_imports)]
    pub use super::log::all::*;
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "sys")]
    pub use super::path::all::*;
}
