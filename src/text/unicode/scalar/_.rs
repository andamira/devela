// devela/src/text/unicode/scalar/_.rs
//
#![doc = crate::_DOC_TEXT_UNICODE_SCALAR!()] // public
#![doc = crate::_doc!(modules: crate::text::unicode; scalar)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: char)]

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod _reexport_core;

    mod_ iter; // CharIter
    mod_ namespace; // Char
    mod offset; // scalar_offset!
    mod_ scalar_; // ch!, char7, char8, char16, charu, charu_niche (WAIT:circular-module)
    mod unicode_scalar; // UnicodeScalar
}
crate::mods_out! { // _mods, _reexports, _hidden
    _mods {
        pub use super::{
            iter::_all::CharIter,
            namespace::_all::Char,
            offset::scalar_offset,
            scalar_::_all::*,
            unicode_scalar::UnicodeScalar,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
    _hidden {
        pub use super::{
            namespace::_hidden::*,
        };
    }
    // _self_internals {} // TODO
}
use scalar_::_crate_internals::{NonSurrogateU16}; // IMPROVE
