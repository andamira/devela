// devela::lang
//
//! Foreign function interfaces and languages bindings.
// #![doc = crate::doc_!(modules: crate; lang: c, py, script, wasm)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ffi)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]

mod reexports;

crate::items! { // structural access:: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::reexports::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
