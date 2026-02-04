// devela_base_alloc::num
//
#![doc = crate::_DOC_NUM!()] // public, root
#![doc = crate::_DOC_NUM_MODULES!()]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: cmp, num, simd)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_NUM_MODULES =
    crate::_doc!(modules: crate; num: dom, prob); // error, fin, grain, lin, quant, symb
}

pub mod dom; // Numeric domains and value representations
pub mod prob; // Probability theory and statistical inference

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            dom::_all::*,
            prob::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_NUM_MODULES;
    }
}
