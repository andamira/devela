// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::code::doc_extends!(arch, env, ffi, fs, io, net, os, path, simd, time)]
//!
//

// safety:
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

#[allow(unused_imports)]
use crate::code::items;

mod arch;
mod env;
mod io;
mod os;
#[allow(unused_imports)]
pub use {arch::*, env::*, io::*, os::all::*};

#[cfg(feature = "log")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "log")))]
items! {
    mod log;
    pub use log::*;
}

#[cfg(feature = "sys")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "sys")))]
items! {
    mod path;
    #[allow(unused_imports)]
    pub use path::*;
}

pub mod ffi;
pub mod time;
#[doc(no_inline)]
pub use {ffi::*, time::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{arch::all::*, env::all::*, ffi::all::*, io::all::*, os::all::*, time::all::*};

    #[doc(inline)]
    #[cfg(feature = "log")]
    #[allow(unused_imports)]
    pub use super::log::all::*;
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "sys")]
    pub use super::path::all::*;
}
