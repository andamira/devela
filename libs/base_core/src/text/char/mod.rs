// devela_base_core::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()]
//

mod _reexport; // SYMLINK from /src/text/char/_reexport_core.rs

mod ascii; // CharAscii
mod digits; // Digits
mod iter; // CharIter
mod namespace; // Char
mod scalar; // char7, char8, char16, charu, charu_niche

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
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
