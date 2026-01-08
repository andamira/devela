// devela_base_core::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
//

mod _reexport; // SYMLINK from /src/num/ord/_reexport_core.rs

mod cmp; // Cmp
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
