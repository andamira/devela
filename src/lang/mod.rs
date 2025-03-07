// devela::lang
//
//! Language functionality, <abbr title = "Domain Specific Language">DSL</abbr>s
//! and <abbr title = "Foreign Function Interface">FFI</abbr>s.
#![doc = crate::doc_!(modules: crate; lang: c, glsl, js)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ffi)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]

pub mod c;

#[cfg(feature = "glsl")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "glsl")))]
pub mod glsl;
#[cfg(feature = "js")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "js")))]
pub mod js;

crate::items! { // structural access:: _mods, _internals, _all, _always
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _pub_mods { #![allow(unused)]
        pub use super::c::_all::*;
        #[cfg(feature = "glsl")]
        pub use super::glsl::_all::*;
        #[cfg(feature = "js")]
        pub use super::js::_all::*;
        // WIPZONE:
        // pub use super::aos::_all::*;
        // pub use super::awk::_all::*;
        // pub use super::py::_all::*;
        // pub use super::script::_all::*;
        // pub use super::tg::_all::*;
    }
    pub(super) mod _internals {
        #[cfg(feature = "js")]
        pub(crate) use super::js::_internals::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
// WIPZONE
// pub mod aos;
// pub mod awk;
// pub mod py;
// pub mod script;
// pub mod tg;
