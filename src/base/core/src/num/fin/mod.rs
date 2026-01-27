// devela_base_core::num::fin
//
#![doc = crate::_DOC_NUM_FIN!()]
//

// mod auto;
mod bit; // BitOps, Bitwise
// mod comb;
// mod graph;
mod ord; // Cmp, cmp!, Order
// mod seq;

pub mod logic; // ConstBool, False, True, const_bool!

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            // auto::_all::*,
            bit::_all::*,
            // comb::_all::*,
            // graph::_all::*,
            ord::_all::*,
            // seq::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            logic::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            ord::_crate_internals::*,
        };
    }
}
