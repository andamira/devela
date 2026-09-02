// devela/src/text/parse/scanner/_.rs
//
//! Defines [`TextScanner`]
//

crate::mods_in! {
    mod define; // TextScanner

    // impls
    mod _helper;
    mod core; // constructors, source views, cursor/range basics, predicate adapters
    mod byte; // byte inspection, exact byte consumption, byte-delimited range scanning
    mod ascii; // ASCII whitespace, identifiers, AsciiSet scanning
    mod number; // ASCII numeric parsing
    mod line; // EOL and line-oriented scanning
    mod quote; // quoted string scanning and decoding
    mod scalar; // UTF-8 scalar scanning
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::TextScanner,
        };
    }
}
