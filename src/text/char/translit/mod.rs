// devela::text::char::transliterate

mod fns; // scalar_as_ascii_translit[_unchecked]
mod table; // ASCII_TRANSLIT

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            fns::*,
        };
    }
    _crate_internals {
        pub use super::{
            table::*,
        };
    }
}
