// devela_base_core::text::parse
//
#![doc = crate::_DOC_TEXT_PARSE!()] // public
#![doc = crate::_doc!(modules: crate::text; parse)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]
//

mod _reexport; // SYMLINK from /src/text/parse/_reexport_core.rs

mod byte_search; // ByteSearch, dep_memchr fns alternatives

// WIPZONE
// mod eval; // eval!, Eval
// mod int; // int_to_str

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            byte_search::*,
        };
        // WIPZONE
        // pub use super::eval::*;
        // pub use super::int::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
