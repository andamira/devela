// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::doc_!(modules: crate; sys: io, os, time)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice, simd, time)]
//
// safety
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

pub mod io;
pub mod log;
pub mod mem;
pub mod os;
#[doc(no_inline)]
#[allow(unused_imports)]
pub use {io::all::*, log::all::*, mem::all::*, os::all::*};

#[cfg(feature = "time")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
    pub mod time;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use time::all::*;
}

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::{arch::all::*, env::all::*, io::all::*, log::all::*, mem::all::*, os::all::*};

    #[doc(inline)]
    #[cfg(feature = "sys")]
    pub use super::path::all::*;
    #[doc(inline)]
    pub use super::sound::all::*;
    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use super::time::all::*;
}
