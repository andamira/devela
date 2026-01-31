// devela_base_core::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()] // public
#![doc = crate::_doc!(modules: crate::text; char)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]
//

mod _reexport; // SYMLINK from /src/text/char/_reexport_core.rs

mod ascii; // CharAscii
mod digits; // Digits
mod iter; // CharIter
mod namespace; // Char
mod scalar; // char7, char8, char16, charu, charu_niche
#[cfg(feature = "translit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "translit")))]
mod translit; // scalar_as_ascii_translit()

// no public re-exports
mod luts;
#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods, _reexports, _workspace_internals
    _mods {
        pub use super::{
            ascii::*,
            digits::*,
            iter::*,
            namespace::*,
            scalar::_all::*,
        };
        #[cfg(feature = "translit")]
        pub use super::translit::_all::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
    _workspace_internals {
        #[cfg(feature = "translit")]
        pub use super::translit::_workspace_internals::*;
    }
}
