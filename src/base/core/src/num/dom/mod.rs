// devela_base_core::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()] // public
#![doc = crate::_doc!(modules: crate::num; dom: int, real)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod _internals; // impl_ops!, upcasted_op! TODO:RENAME

// mod complex;
// mod fract;
// mod laws;
// mod ops;
mod traits; // NumConst

pub mod int; // Divisor, GcdReturn, Int, [i|u]size_[down|up]
pub mod real; // Real-valued numeric domains and representations.

crate::structural_mods! { // _mods, _pub_mods, _workspace_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // complex::_all::*,
            // fract::_all::*,
            // laws::_all::*,
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
    _workspace_internals {
        pub use super::{
            _internals::*,
            real::_workspace_internals::*,
            int::_workspace_internals::*,
        };
    }
    _hidden {
        pub use super::int::_hidden::*;
    }
}
