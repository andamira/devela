// devela::text::parse
//
//! String parsing without structured semantics.
//

mod byte_search; // ByteSearch, dep_memchr fns alternatives
mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{byte_search::*, reexports::*};
        // WIPZONE
        // pub use super::eval::*;
        // pub use super::int::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
// WIPZONE
// mod eval; // eval!, Eval
// mod int; // int_to_str
