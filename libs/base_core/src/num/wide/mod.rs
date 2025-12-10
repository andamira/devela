// devela_base_core::num::wide
//
#![doc = crate::_DOC_NUM_WIDE!()]
//

mod _dep_wide; // _dep_wide_compile!, _dep_wide_use!

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
        pub use super::_dep_wide::*;
    }
}
