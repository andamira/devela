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

// structural access
crate::items! {
    #[allow(unused_imports)]
    pub use {doc_inline::*, doc_hidden::*, items_hidden::*};

    mod doc_inline {
        pub use super::{arch::all::*, env::all::*, io::all::*, mem::all::*};
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::{log::all::*, os::all::*, sound::all::*};
        #[cfg(feature = "sys")]
        pub use super::path::all::*;
    }
    mod doc_hidden { #[doc(hidden)] #[doc(no_inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::{
            io::all::*, log::all::*, mem::all::*, os::all::*,
        };
    }
    pub(super) mod items_hidden { pub use super::mem::items_hidden::*; }
    pub(super) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::{doc_inline::*, doc_hidden::*};
    }
    pub(super) mod always { #![allow(unused_imports)]
        pub use super::{
            arch::always::*, env::always::*, io::always::*, mem::always::*,
        };
    }
}
