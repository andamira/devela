// devela::text::parse
//
//! String parsing without structured semantics.
//

#[cfg(test)]
mod tests;

mod _reexport_core;

mod byte_search; // ByteSearch, dep_memchr fns alternatives
mod error; // TextParseError[Kind]
mod scanner; // TextScanner

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            byte_search::*,
            error::*,
            scanner::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
