// devela::text::str
//
#![doc = crate::_DOC_TEXT_STR!()] // public
#![doc = crate::_doc!(modules: crate::text; str)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: str, string)]

mod _reexport_core;
#[cfg(feature = "alloc")]
mod _reexport_alloc;
#[cfg(feature = "std")]
mod _reexport_std;

mod buf; // WIP StrBuf
mod ext; // StrExt, StringExt
mod namespace; // Str
mod nonul; // StringNonul
// mod #[cfg(feature = "alloc")]
// mod small; // WIP StringSmall
mod u; // StringU8, StringU16

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            buf::*,
            ext::*,
            namespace::Str,
            nonul::*,
            u::*,
        };
        // mod #[cfg(feature = "alloc")]
        // pub use super::small::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;

        // from other modules
        pub use crate::CStr;
        #[cfg(feature = "alloc")]
        pub use crate::CString;
    }
}
