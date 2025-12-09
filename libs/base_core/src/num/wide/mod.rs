// devela_base_core::num::wide
//
#![doc = crate::_DOC_NUM_WIDE!()]
//

mod lane; // define_lane!

mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            lane::*,
        };

        pub use super::reexports::*;
    }
}
