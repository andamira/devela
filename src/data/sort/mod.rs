// devela::data::sort
//
//! Sorting functionality.
//

// no items defined
mod generic;
#[cfg(_sort_·)]
mod primitives;

mod definition;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::definition::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
