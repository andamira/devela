// devela::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()]
// #![doc = crate::_doc!(extends: char)]
// #![doc = crate::_doc!(modules: crate::text; char)]
// #![doc = crate::_doc!(newline)]

mod unicode_scalar; // UnicodeScalar

// with re-exports
crate::mod_path!(_c "../../../libs/base_core/src/text/char/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use super::unicode_scalar::*;

        pub use super::_c::*; // char

        #[doc(inline)]
        pub use devela_base_core::text::{
            LUT_ASCII_CHARS, LUT_DIGITS_BASE36, LUT_POWERS10,
            Char, CharAscii, CharIter, Digits, char7, char8, char16, char_utf8,
        };

        #[cfg(feature = "devela_base_text")]
        pub use devela_base_text::{
            scalar_as_ascii_translit,
        };
    }
}

mod impls {
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_layout"))]
    unsafe impl crate::MemPod for crate::char8 {}
}
