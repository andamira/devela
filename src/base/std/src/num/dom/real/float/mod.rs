// devela_base_std::num::dom::real::float
//
#![doc = crate::_DOC_NUM!()]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]

mod wrapper; // FloatStd

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            wrapper::_all::*,}
        ;
    }
}
