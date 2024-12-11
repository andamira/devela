// devela::sys
//
//! System interfaces and hardware abstractions.
#![doc = crate::doc_!(modules: crate; sys: io, mem, os)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice, simd)]
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

crate::items! { // structural access: doc_inline, doc_hidden, items_hidden, all, always
    #[allow(unused)]
    pub use {doc_inline::*, doc_hidden::*, items_hidden::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline { #![allow(unused)]
        pub use super::{arch::all::*, env::all::*, sound::all::*};
        #[cfg(feature = "sys")]
        pub use super::path::all::*;
    }
    mod doc_hidden { #![allow(unused)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{io::all::*, log::all::*, mem::all::*, os::all::*};
    }
    pub(super) mod items_hidden {
        pub use super::mem::items_hidden::*;
    }
    pub(super) mod all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{doc_inline::*, doc_hidden::*};
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{
            arch::always::*, env::always::*, io::always::*, mem::always::*,
        };
    }
}
