// devela/src/text/parse/mod.rs
//
//! String parsing without structured semantics.
//

mod _reexport_core;

mod byte_search; // ByteSearch, dep_memchr fns alternatives
mod error; // TextParseError[Kind]
mod scanner; // TextScanner

crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            byte_search::*,
            error::*,
            scanner::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
