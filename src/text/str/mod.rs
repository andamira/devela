// devela::text::str
//
#![doc = crate::_DOC_TEXT_STR!()] // public
#![doc = crate::_doc!(modules: crate::text; str)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: str, string)]

mod _reexport_core; // SYMLINK to /src/base/core/src/text/str/_reexport.rs
mod _reexport_alloc; // SYMLINK to /src/base/alloc/src/text/str/_reexport.rs
mod _reexport_std; // SYMLINK to /src/base/std/src/text/str/_reexport.rs

mod ext_str; // StrExt

#[cfg(feature = "alloc")]
mod ext_string; // StringExt

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            ext_str::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::ext_string::*;
    }
    _reexports {
        pub use super::{
            _reexport_core::*,
            _reexport_alloc::*,
            _reexport_std::*,
        };
        #[doc(inline)]
        pub use devela_base_core::text::str::{
            Str, StringNonul, StringU8, StringU16, StringU32, StringUsize,
        };
        // from other modules
        pub use crate::CStr;
        #[cfg(feature = "alloc")]
        pub use crate::CString;
    }
}
