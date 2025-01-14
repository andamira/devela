// devela::lang
//
//! Language functionality, <abbr title = "Domain Specific Language">DSL</abbr>s
//! and <abbr title = "Foreign Function Interface">FFI</abbr>s.
#![doc = crate::doc_!(modules: crate; lang: c, /* js, py, script, wasm*/ )]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ffi)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]

pub mod c;

crate::items! { // structural access:: _mods, _all, _always
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};
    // #[allow(unused)] #[doc(hidden, no_inline)]
    // pub use _always::*;

    mod _pub_mods {
        pub use super::c::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
