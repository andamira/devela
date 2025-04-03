// devela::data::list::of
//
//!
//

mod oneof; // Oneof

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::oneof::*;
        // WIPZONE
        // pub use super::allof::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod allof;
