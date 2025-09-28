// devela_base_core::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()]
//

mod chars; // IterChars
mod definitions; // char7, char8, char16, char_utf8
mod namespace; // Char
mod reexports;
mod unicode_scalar; // UnicodeScalar

// without re-exports
mod impls;
#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            chars::*,
            definitions::*,
            namespace::*,
            reexports::*,
            unicode_scalar::*,
        };
    }
}
