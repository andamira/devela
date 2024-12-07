// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::doc_!(modules: crate; sys: io, mem, os, time)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice, simd, time)]
//
// safety
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

mod arch;
mod env;
mod sound; // IMPROVE

#[cfg(feature = "sys")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "sys")))]
mod path;

pub mod io;
pub mod log;
pub mod mem;
pub mod os;

#[cfg(feature = "time")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
pub mod time;

// structural access
crate::items! {
    mod doc_inline {
        #![expect(unused_imports, reason = "WIP")]
        pub use super::{
            arch::all::*, env::all::*, io::all::*, log::all::*, mem::all::*, os::all::*,
        };
        pub use super::sound::all::*;
        #[cfg(feature = "sys")]
        pub use super::path::all::*;
        #[cfg(feature = "time")]
        pub use super::time::all::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
