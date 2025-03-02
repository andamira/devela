// devela::num::logic
//
//! Logic related types and functionality.
//

mod bool; // ConstBool, False, True

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::bool::*;
        // WIPZONE
        // pub use super::bops::*;
        // pub use super::choice::*;
        // pub use super::items::*;
        // pub use super::linear::*;
        // pub use super::trool::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod bops;
// mod choice;
// mod items;
// mod linear;
// mod trool;
