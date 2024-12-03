// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::doc_!(modules: crate; sys: ffi, io, os, time)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: arch, env, ffi, fs, io, net, os, path, simd, time)]
//

// safety:
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

#[allow(unused_imports)]
use crate::items;

mod arch;
mod env;
#[allow(unused_imports)]
pub use {arch::*, env::*};

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

/* public */

#[cfg(feature = "time")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
    pub mod time;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use time::all::*;
}

pub mod ffi;
pub mod io;
pub mod log;
pub mod os;
#[doc(no_inline)]
#[allow(unused_imports)]
pub use {ffi::all::*, io::all::*, os::all::*};

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::{arch::all::*, env::all::*, ffi::all::*, io::all::*, log::all::*, os::all::*};

    #[doc(inline)]
    #[cfg(feature = "sys")]
    pub use super::path::all::*;
    #[doc(inline)]
    pub use super::sound::all::*;
    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use super::time::all::*;
}
