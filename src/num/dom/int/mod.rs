// devela::num::dom::int
//
#![doc = crate::_DOC_NUM_DOM_INT!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

pub(crate) mod _docs; // _INT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()

mod alias; // [i|u]size_[down|up]
mod divisor; // divisor!, DivisorExample, (DivisorInner)
mod fns; // prime_number_teorem()
mod gcd; // GcdReturn
// mod prim; // i256, u256 WIP RENAME
// mod primes; // WIP
// mod recip; // DivRecip WIP

#[cfg(feature = "int")]
mod num_trait; // NumInt, NumRefInt
#[cfg(feature = "int")]
mod wrapper; // Int, int! WIP

crate::structural_mods! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            alias::*,
            divisor::divisor,
            fns::*,
            gcd::*,
            // prim::*,
            // recip::*,
        };
        #[cfg(feature = "int")]
        pub use super::{
            num_trait::*,
            wrapper::_all::*,
        };
        #[cfg(feature = "_docs_examples")]
        pub use super::divisor::DivisorExample;
    }
    _crate_internals {
        #[cfg(feature = "int")]
        pub use super::_docs::*;
    }
    _hidden {
        pub use super::divisor::DivisorInner;
    }
}
