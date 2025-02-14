// devela::num::logic
//
//! Logic related types and functionality.
//

mod bool;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::bool::*;
        // WIPZONE
        // pub use super::binary::*;
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
// mod binary;
// mod choice;
// mod items;
// mod linear;
// mod trool;
