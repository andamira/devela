// devela_base::num
//
//! Numerical types and math operations.
//
// safety
// #![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod float;
mod int;
// mod primitive; // Cast, Primitive[Cast|Join|Split]

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{float::_all::*, int::_all::*};
        // pub use super::primitive::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
