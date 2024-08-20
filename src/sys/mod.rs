// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::code::doc_extends!(arch, env, ffi, fs, io, net, os, path, simd, time)]
//!
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

pub mod time;
pub use time::*;

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

    #[doc(inline)]
    pub use super::time::all::*;
}
