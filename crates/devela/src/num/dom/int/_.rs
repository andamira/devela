//
#![doc = crate::_DOC_NUM_DOM_INT!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    pub(crate) mod _docs; // _DOC_INT_[ALGORITHM|FORMULA|NOTATION|PIECEWISE]_*!()

    mod alias; // [i|u]size_[down|up]

    mod_ divisor; // divisor!, DivisorExample
    mod fns; // prime_number_teorem() TEMP
    mod gcd; // GcdReturn
    #[cfg(feature = "int")]
    mod_ r#macro; // int!, IntExample

    // mod_ prim; // i256, u256 WIP RENAME
    // mod_ primes; // TODO
    // mod recip; // DivRecip WIP

    #[cfg(feature = "int")]
    mod_ wrapper; // Int, TODO: int!
}
crate::mods_out! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            alias::*,
            divisor::_all::*,
            fns::prime_number_theorem,
            gcd::GcdReturn,
            // prim::*,
            // recip::*,
        };
        #[cfg(feature = "int")]
        pub use super::{
            r#macro::int,
            wrapper::_all::Int,
        };
        #[cfg(all(feature = "int", feature = "_docs_examples"))]
        pub use super::r#macro::IntExample;
    }
    _crate_internals {
        #[cfg(feature = "int")]
        pub(crate) use super::_docs::*;
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            divisor::_hidden::*,
        };
        #[doc(hidden)]
        #[cfg(feature = "int")]
        pub use super::r#macro::_hidden::*;
    }
}
