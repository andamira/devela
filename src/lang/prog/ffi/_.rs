// devela/src/lang/prog/ffi/_.rs
//
#![doc = crate::_DOC_LANG_PROG_FFI!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; ffi: c, glsl, js)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//!
//! Foreign interfaces adapt representations and conventions defined by
//! external languages, ABIs, and platforms.
//!
//! Types and bindings here may intentionally mirror foreign layouts or naming
//! where interoperability requires it. They do not define devela's internal
//! semantic model.
//
// - Use repr(u8) for small, FFI-safe enums (C-like, no fields).
// - Use repr(C) for structs that contain repr(u8) enums.
// - Never use repr(u8) if the enum has fields—use repr(C) instead.

crate::mods_in! {
    pub mod_ c;

    #[cfg(feature = "glsl")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "glsl")))]
    pub mod_ glsl;
    #[cfg(all(feature = "js", not(windows)))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
    pub mod_ js; // javascript

    // pub mod_ aos; // android
    // pub mod_ py; // python
    // pub mod_ tg; // telegram
}
crate::mods_out! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::c::_all::*;
        #[cfg(feature = "glsl")]
        pub use super::glsl::_all::*;
        #[cfg(all(feature = "js", not(windows)))]
        pub use super::js::_all::*;

        // pub use super::aos::_all::*;
        // pub use super::py::_all::*;
        // pub use super::tg::_all::*;
    }
    _crate_internals {
        #[cfg(all(feature = "js", not(windows)))]
        pub(crate) use super::js::_crate_internals::*;
    }
}
