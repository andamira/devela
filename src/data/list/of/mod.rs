// devela::data::list::of
//
//!
//

mod oneof; // Oneof

// WIPZONE
// mod allof;

oneof::impl_oneof!(impl_const_default);

crate::structural_mods! { // _mods
    _mods {
        pub use super::oneof::*;
        // WIPZONE
        // pub use super::allof::*;
    }
}
