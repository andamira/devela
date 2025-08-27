// devela::text::parse
//
//! String parsing without structured semantics.
//

crate::mod_path!(_c "../../../libs/base/src/text/parse/reexports.rs");

mod byte_search; // ByteSearch, dep_memchr fns alternatives

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{_c::*, byte_search::*};
        // WIPZONE
        // pub use super::eval::*;
        // pub use super::int::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::_c::*;
    }
}
// WIPZONE
// mod eval; // eval!, Eval
// mod int; // int_to_str
