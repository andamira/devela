// devela/src/text/str/mod.rs
//
#![doc = crate::_DOC_TEXT_STR!()] // public
#![doc = crate::_doc!(modules: crate::text; str)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: str, string)]

mod _reexport;

mod array; // StringNonul, StringU8, StringU16
mod buf; // StrBuf
mod ext; // StrExt, StringExt
mod namespace; // Str
mod small; // StringSmallAlloc

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            array::_all::*,
            buf::*,
            ext::*,
            namespace::Str,
            small::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;

        // from other modules
        pub use crate::CStr;
        #[cfg(feature = "alloc")]
        pub use crate::CString;
    }
}
