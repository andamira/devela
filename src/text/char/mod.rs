// devela::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()] // public
#![doc = crate::_doc!(modules: crate::text; char)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: char)]

mod _reexport_core; // SYMLINK to /src/base/core/src/text/char/_reexport.rs

mod unicode_scalar; // UnicodeScalar

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::unicode_scalar::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc(inline)]
        pub use devela_base_core::text::{
            Char, CharAscii, CharIter, Digits, ch, char7, char8, char16, charu, charu_niche,
        };
        #[cfg(feature = "translit")]
        pub use devela_base_core::text::scalar_as_ascii_translit;
    }
}

mod impls {
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_layout"))]
    unsafe impl crate::MemPod for crate::char8 {}
}
