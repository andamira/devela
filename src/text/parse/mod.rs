// devela::text::parse
//
//! String parsing without structured semantics.
//

crate::mod_path!(_c "../../../libs/base/src/text/parse/reexports.rs");

mod byte_search; // ByteSearch, dep_memchr fns alternatives

// WIPZONE
// mod eval; // eval!, Eval
// mod int; // int_to_str

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{_c::*, byte_search::*};

        // WIPZONE
        // pub use super::eval::*;
        // pub use super::int::*;
    }
    _always {
        pub use super::_c::*;
    }
}
