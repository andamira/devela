// devela_base_core::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()]
//

// mod complex;
mod int; // Divisor, GcdReturn, Int, [i|u]size_[down|up]
// mod fract;
// mod laws;
// mod ops;
mod real; // Real-valued numeric domains and representations.
mod traits; // NumConst

crate::structural_mods! { // _mods, _workspace_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // complex::_all::*,
            int::_all::*,
            // fract::_all::*,
            // laws::_all::*,
            // ops::_all::*,
            real::_all::*,
            traits::_all::*,
        };
    }
    _workspace_internals {
        pub use super::{
            real::_workspace_internals::*,
            int::_workspace_internals::*,
        };
    }
    _hidden {
        pub use super::int::_hidden::*;
    }
}
