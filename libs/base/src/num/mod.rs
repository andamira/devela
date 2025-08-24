// devela_base::num
//
#![doc = crate::_DOC_NUM!()]
//
// safety
// #![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod float;
mod int;
// mod cast; // Cast
// TODO: Needs Overflow error

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{float::_all::*, int::_all::*};
        // pub use super::cast::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
