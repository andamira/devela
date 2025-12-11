// devela_base_core::num::wide
//
#![doc = crate::_DOC_NUM_WIDE!()]
//!
#![doc = crate::_doc!(extends: simd)]
//

mod _helpers; // __lane_dispatch!, _dep_wide_compile!, _dep_wide_use!

mod lane; // define_lane!

mod reexports;

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            lane::*,
        };

        pub use super::reexports::*;
    }
    _hidden {
        pub use super::_helpers::*;
    }
}
