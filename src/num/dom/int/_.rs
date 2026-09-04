// devela/src/num/dom/int/_.rs
//
#![doc = crate::_DOC_NUM_DOM_INT!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    pub(crate) mod _docs; // _DOC_INT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()

    mod alias; // [i|u]size_[down|up]
    mod divisor; // divisor!, DivisorExample, (DivisorInner)
    mod fns; // prime_number_teorem() TEMP
    mod gcd; // GcdReturn
    // mod_ prim; // i256, u256 WIP RENAME
    // mod_ primes; // TODO
    // mod recip; // DivRecip WIP

    #[cfg(all(feature = "int", feature = "num"))]
    mod_ num_trait; // NumInt, NumRefInt TEMP
    #[cfg(feature = "int")]
    mod_ wrapper; // Int, TODO: int!
}
crate::mods_out! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            alias::*,
            divisor::divisor,
            fns::prime_number_theorem,
            gcd::GcdReturn,
            // prim::*,
            // recip::*,
        };
        #[cfg(all(feature = "int", feature = "num"))]
        pub use super::num_trait::*;
        #[cfg(feature = "int")]
        pub use super::wrapper::_all::Int;
        #[cfg(feature = "_docs_examples")]
        pub use super::divisor::DivisorExample;
    }
    _crate_internals {
        #[cfg(feature = "int")]
        pub(crate) use super::_docs::*;
    }
    _hidden {
        pub use super::divisor::DivisorInner;
    }
}
