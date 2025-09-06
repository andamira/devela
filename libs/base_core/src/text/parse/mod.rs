// devela_base_core::text::parse
//
//! String parsing without structured semantics.
//

mod byte_search; // ByteSearch, dep_memchr fns alternatives

mod reexports;

// WIPZONE
// mod eval; // eval!, Eval
// mod int; // int_to_str

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            byte_search::*,
            reexports::*,
        };

        // WIPZONE
        // pub use super::eval::*;
        // pub use super::int::*;
    }
}
