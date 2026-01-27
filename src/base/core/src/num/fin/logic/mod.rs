// devela_base_core::num::fin::logic
//
#![doc = crate::_DOC_NUM_FIN_LOGIC!()]
//

mod bool; // ConstBool, False, True
// mod bops;
// mod choice;
// mod items;
// mod linear;
// mod trool;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            bool::*,
            // bops::*;
            // choice::*;
            // items::*;
            // linear::*;
            // trool::*;
        };
    }
}
