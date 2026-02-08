// devela_base_core::num::fin::ord
//
#![doc = crate::_DOC_NUM_FIN_ORD!()] // public
#![doc = crate::_doc!(modules: crate::num::fin; ord)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod _reexport; // SYMLINK from /src/num/fin/ord/_reexport_core.rs

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
        pub use super::_reexport::*;
    }
    _crate_internals {
        pub(crate) use super::order::_crate_internals::*;
    }
}
