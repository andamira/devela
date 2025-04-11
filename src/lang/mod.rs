// devela::lang
//
//! Language functionality, <abbr title = "Domain Specific Language">DSL</abbr>s
//! and <abbr title = "Foreign Function Interface">FFI</abbr>s.
#![doc = crate::doc_!(modules: crate; lang: dsl, ffi, i18n, ling)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ffi)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]

pub mod dsl; // forth, …
pub mod ffi; // c, glsl, js, …
pub mod i18n; // gettext, fluent, …
pub mod ling; // art, nat, … (linguistics)

crate::items! { // structural access:: _pub_mods, _internals, _all, _always
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _pub_mods { #![allow(unused)]
        pub use super::ffi::_all::*;
        pub use super::dsl::_all::*;
        pub use super::i18n::_all::*;
        pub use super::ling::_all::*;
        // WIPZONE
        // pub use super::script::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::ffi::_internals::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)] }
}
// WIPZONE
// mod script;
