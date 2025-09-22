// devela_base_core::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
//

mod cmp; // Cmp
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cmp::*,
            reexports::*,
        };
    }
}
