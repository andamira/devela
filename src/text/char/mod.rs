// devela::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()] // public
#![doc = crate::_doc!(modules: crate::text; char)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: char)]

mod _reexport_core;

mod ascii; // AsciiSet, CharAscii
mod digits; // Digits
mod iter; // CharIter
mod namespace; // Char
mod scalar; // ch!, char7, char8, char16, charu, charu_niche
#[cfg(feature = "translit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "translit")))]
mod translit; // scalar_as_ascii_translit()
mod unicode_scalar; // UnicodeScalar

// no public re-exports
mod luts;
#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            ascii::*,
            digits::*,
            iter::*,
            namespace::*,
            scalar::_all::*,
            unicode_scalar::*,
        };
        #[cfg(feature = "translit")]
        pub use super::translit::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
    _crate_internals {
        #[cfg(feature = "translit")]
        pub use super::translit::_crate_internals::*;
    }
}

mod impls {
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_layout"))]
    unsafe impl crate::MemPod for crate::char8 {}
}
