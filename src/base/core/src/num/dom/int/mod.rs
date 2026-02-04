// devela_base_core::num::dom::int
//
#![doc = crate::_DOC_NUM_DOM_INT!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod alias; // [i|u]size_[down|up]
mod divisor; // define_divisor!, DivisorExample, (DivisorInner)
mod gcd; // GcdReturn

#[cfg(feature = "int")]
mod int; // Int, define_int! WIP
// mod prim; // i256, u256 WIP
// mod recip; // DivRecip WIP

crate::structural_mods! { // _mods, _workspace_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            alias::*,
            divisor::define_divisor,
            gcd::*,
            // prim::*, // WIP
            // recip::*, // WIP
        };
        #[cfg(feature = "int")]
        pub use super::int::_all::*;
        #[cfg(feature = "_docs_examples")]
        pub use super::divisor::DivisorExample;
    }
    _workspace_internals {
        #[cfg(feature = "int")]
        pub use super::int::_workspace_internals::*;
    }
    _hidden {
        pub use super::divisor::DivisorInner;
    }
}
