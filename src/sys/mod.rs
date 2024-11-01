// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::doc_!(extends: arch, env, ffi, fs, io, net, os, path, simd, time)]
#![doc = crate::doc_!(modules: crate; sys: ffi, io, os, time)]
#![doc = crate::doc_!(newline)]
//!
//

// safety:
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

#[allow(unused_imports)]
use crate::code::items;

mod arch;
mod env;
#[allow(unused_imports)]
pub use {arch::*, env::*};

#[cfg(feature = "dep_log")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_log")))]
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
items! {
    mod sound;
    #[allow(unused_imports)]
    pub use sound::*;
}

pub mod ffi;
pub mod io;
pub mod os;
pub mod time;
#[doc(no_inline)]
#[allow(unused_imports)]
pub use {ffi::*, io::all::*, os::all::*, time::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{arch::all::*, env::all::*, ffi::all::*, io::all::*, os::all::*, time::all::*};

    #[doc(inline)]
    #[cfg(feature = "dep_log")]
    #[allow(unused_imports)]
    pub use super::log::all::*;
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "sys")]
    pub use super::path::all::*;
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::sound::all::*;
}
