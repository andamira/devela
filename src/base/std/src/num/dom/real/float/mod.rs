// devela_base_std::num::dom::real::float
//
#![doc = crate::_DOC_NUM_DOM_REAL_FLOAT!()] // private
#![doc = crate::_doc!(modules: crate::num::dom::real; float)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod wrapper; // FloatStd

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            wrapper::_all::*,}
        ;
    }
}
