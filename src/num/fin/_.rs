// devela/src/num/fin/_.rs
//
#![doc = crate::_DOC_NUM_FIN!()] // public
#![doc = crate::_doc!(modules: crate::num; fin: logic, ord)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod auto;
    mod_ bit; // BitOps, Bitwise
    // mod_ comb;
    // mod_ graph;
    pub mod_ logic; // ConstBool, False, True, const_bool!
    pub mod_ ord; // Cmp, cmp!, Order
    // mod rank;
    // mod_ seq;
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            // auto::_all::*,
            bit::_all::*,
            // comb::_all::*,
            // graph::_all::*,
            // rank::*,
            // seq::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            logic::_all::*,
            ord::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            ord::_crate_internals::*,
        };
    }
}
