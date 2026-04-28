// devela::text::parse::scanner
//
//! Defines [`TextScanner`]
//

// impls
mod core; // constructors, source views, cursor/range basics, predicate adapters
mod byte; // byte inspection, exact byte consumption
mod until; // byte-delimited range scanning
mod ascii; // ASCII whitespace, identifiers, AsciiSet scanning
mod number; // ASCII numeric parsing
mod line; // EOL and line-oriented scanning
mod quote; // quoted string scanning and decoding

mod define; // TextScanner

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
