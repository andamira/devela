// devela/src/text/unicode/scalar/mod.rs
//
#![doc = crate::_DOC_TEXT_UNICODE_SCALAR!()] // public
#![doc = crate::_doc!(modules: crate::text::unicode; scalar)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: char)]

#[cfg(test)]
mod tests;

mod _reexport_core;

mod iter; // CharIter
mod namespace; // Char
mod scalar; // ch!, char7, char8, char16, charu, charu_niche
mod unicode_scalar; // UnicodeScalar

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            iter::*,
            namespace::_all::*,
            scalar::_all::*,
            unicode_scalar::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}

mod impls {
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_layout"))]
    unsafe impl crate::MemPod for crate::char8 {}
}
