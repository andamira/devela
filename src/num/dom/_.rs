// devela/src/num/dom/_.rs
//
#![doc = crate::_DOC_NUM_DOM!()] // public
#![doc = crate::_doc!(modules: crate::num; dom: int, real)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
        mod _helper; // _num_dom_impl_arith!, _num_dom_upcast_arith!, _num_dom_upcasted_mul_add!
        // mod_ complex; // TODO
        mod_ frac;
        pub mod_ int; // Divisor, GcdReturn, Int[Alloc], [i|u]size_[down|up], NumInt[Ref], …
        // mod_ laws; // TODO
        mod no; // NoNum
        // mod_ ops; // TODO
    pub mod_ real; // Real-valued numeric domains and representations.
        mod_ traits; // Num, NumRef, NumConst
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // complex::_all::*,
            frac::_all::*,
            // laws::_all::*,
            no::NoNum,
            // ops::_all::*,
            traits::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            int::_all::*,
            real::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            real::_crate_internals::*,
            int::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            _helper::{_num_dom_impl_arith, _num_dom_upcast_arith, _num_dom_upcasted_mul_add},
            int::_hidden::*,
        };
    }
}
