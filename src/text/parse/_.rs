// devela/src/text/parse/_.rs
//
//! String parsing without structured semantics.
//

crate::mods_in! {
    mod _reexport_core;

    mod byte_search; // ByteSearch, dep_memchr fns alternatives
    mod error; // TextParseError[Kind]
    mod_ scanner; // TextScanner
}
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
