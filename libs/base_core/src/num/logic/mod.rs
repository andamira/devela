// devela_base_core::num::logic
//
//! Logic related types and functionality.
//

mod bool; // ConstBool, False, True

// WIPZONE
// mod bops;
// mod choice;
// mod items;
// mod linear;
// mod trool;

crate::structural_mods! { // _mods
    _mods {
        pub use super::bool::*;

        // WIPZONE
        // pub use super::bops::*;
        // pub use super::choice::*;
        // pub use super::items::*;
        // pub use super::linear::*;
        // pub use super::trool::*;
    }
}
