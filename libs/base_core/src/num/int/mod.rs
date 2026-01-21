// devela_base_core::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

mod alias; // [i|u]size_[down|up]
mod divisor; // define_divisor!, DivisorExample, (DivisorInner)
mod gcd; // GcdReturn
// mod int; // define_int! WIP
// mod prim; // i256, u256 WIP
// mod recip; // DivRecip WIP

crate::structural_mods! { // _mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            alias::*,
            divisor::define_divisor,
            gcd::*,
            // int::*, // WIP
            // prim::*, // WIP
            // recip::*, // WIP
        };
        #[cfg(feature = "_docs_min")]
        pub use super::divisor::DivisorExample;
    }
    _hidden {
        pub use super::divisor::DivisorInner;
    }
}
