// devela/src/text/str/_.rs
//
#![doc = crate::_DOC_TEXT_STR!()] // public
#![doc = crate::_doc!(modules: crate::text; str)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: str, string)]

crate::mods_in! {
    mod _reexport;

    mod_ array; // StringNonNul, StringU8, StringU16
    mod buf; // StrBuf
    mod_ ext; // StrExt, StringExt
    mod_ namespace; // Str
    mod_ small; // StringSmallAlloc
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            array::_all::*,
            buf::*,
            ext::_all::*,
            namespace::_all::Str,
            small::_all::*,
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
