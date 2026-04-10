// devela::num::dom::int
//
#![doc = crate::_DOC_NUM_DOM_INT!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod fns; // prime_number_teorem

mod alias; // [i|u]size_[down|up]
mod divisor; // define_divisor!, DivisorExample, (DivisorInner)
mod gcd; // GcdReturn

#[cfg(feature = "int")]
mod int; // Int, define_int! WIP
// mod prim; // i256, u256 WIP RENAME
// mod primes; // WIP
// mod recip; // DivRecip WIP

#[cfg(feature = "int")]
mod num_trait; // NumInt, NumRefInt

crate::structural_mods! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            alias::*,
            divisor::define_divisor,
            fns::*,
            gcd::*,
            // prim::*,
            // recip::*,
        };
        #[cfg(feature = "int")]
        pub use super::{
            int::_all::*,
            num_trait::*,
        };
        #[cfg(feature = "_docs_examples")]
        pub use super::divisor::DivisorExample;
    }
    _crate_internals {
        #[cfg(feature = "int")]
        pub use super::int::_crate_internals::*;
    }
    _hidden {
        pub use super::divisor::DivisorInner;
    }
}
