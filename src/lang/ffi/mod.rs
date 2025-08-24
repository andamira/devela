// devela::lang::ffi
//
#![doc = crate::_DOC_LANG_FFI!()]
#![doc = crate::doc_!(modules: crate::lang; ffi: c, glsl, js)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ffi)]
//
// - Use repr(u8) for small, FFI-safe enums (C-like, no fields).
// - Use repr(C) for structs that contain repr(u8) enums.
// - Never use repr(u8) if the enum has fields—use repr(C) instead.

pub mod c;

#[cfg(feature = "glsl")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "glsl")))]
pub mod glsl;
#[cfg(all(feature = "js", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
pub mod js; // javascript

crate::items! { // structural access: _pub_mods, _internals, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods { #![allow(unused)]
        pub use super::c::_all::*;
        #[cfg(feature = "glsl")]
        pub use super::glsl::_all::*;
        #[cfg(all(feature = "js", not(windows)))]
        pub use super::js::_all::*;
        // WIPZONE:
        // pub use super::aos::_all::*;
        // pub use super::py::_all::*;
        // pub use super::tg::_all::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        #[cfg(all(feature = "js", not(windows)))]
        pub(crate) use super::js::_internals::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
// WIPZONE
// pub mod aos; // android
// pub mod py; // python
// pub mod tg; // telegram
