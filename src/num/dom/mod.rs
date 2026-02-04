// devela::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()] // public
#![doc = crate::_doc!(modules: crate::num; dom: int, real)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

// mod complex;
mod frac;
// mod laws;
// mod ops;

#[cfg(feature = "num")]
mod traits; // Num, NumRef, (NumConst)

pub mod int; // NumInt[Ref], prime_number_theorem, (Divisor, GcdReturn, Int[Alloc], [i|u]size_[down|up])
pub mod real; // Real-valued numeric domains and representations.

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // complex::_all::*,
            frac::_all::*,
            // laws::_all::*,
            // ops::_all::*,
        };
        #[cfg(feature = "num")]
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
}
