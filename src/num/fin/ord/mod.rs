// devela::num::fin::ord
//
#![doc = crate::_DOC_NUM_FIN_ORD!()] // public
#![doc = crate::_doc!(modules: crate::num::fin; ord)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: cmp)]
//

mod _reexport_core;

mod cmp; // Cmp, cmp!
mod order; // Order

crate::structural_mods! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            cmp::*,
            order::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
    _crate_internals {
        pub(crate) use super::order::_crate_internals::*;
    }
}
