// devela::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()] // public
#![doc = crate::_doc!(modules: crate::num; dom: int, real)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod _helper; // _num_dom_impl_arith!, _num_dom_upcast_arith!, _num_dom_upcasted_mul_add!

// mod complex;
mod frac;
// mod laws;
mod no; // NoNum
// mod ops;

mod traits; // Num, NumRef, NumConst

pub mod int; // Divisor, GcdReturn, Int[Alloc], [i|u]size_[down|up], NumInt[Ref], prime_number_theorem
pub mod real; // Real-valued numeric domains and representations.

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // complex::_all::*,
            frac::_all::*,
            // laws::_all::*,
            no::*,
            // ops::_all::*,
        };
        pub use super::{
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
        pub use super::{
            real::_crate_internals::*,
            int::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            _helper::*,
            int::_hidden::*,
        };
    }
}
