// devela_base::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()]
//

mod definitions; // UnicodeScalar, char7, char8, char16
mod namespace; // Char
mod reexports;

// without re-exports
mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definitions::*,
            namespace::*,
            reexports::*,
        };
    }
}
