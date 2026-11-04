//
#![doc = crate::_DOC_NUM_DOM!()] // public
#![doc = crate::_doc!(modules: crate::num; dom: complex, fixed, float, frac, int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
        mod _helper; // _num_dom_impl_arith!, _num_dom_upcast_arith!, _num_dom_upcasted_mul_add!
    pub mod_ complex; // Complex-valued numeric representations (WIP)
    pub mod_ fixed; // Fixed-point numeric representations (WIP)
    pub mod_ float; // Floating-point numeric representations
    pub mod_ frac; // Fractional numeric representations
    pub mod_ int; // Integer types and operations
    // mod_ laws; // TODO
    mod no; // NoNum
    // mod_ ops; // TODO
    mod_ traits; // NumConst
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // laws::_all::*,
            no::NoNum,
            // ops::_all::*,
            traits::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            complex::_all::*,
            fixed::_all::*,
            float::_all::*,
            frac::_all::*,
            int::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            float::_crate_internals::*,
            int::_crate_internals::*,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            _helper::{_num_dom_impl_arith, _num_dom_upcast_arith, _num_dom_upcasted_mul_add},
            int::_hidden::*,
        };
    }
}
