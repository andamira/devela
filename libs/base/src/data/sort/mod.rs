// devela_base::data::sort
//
//! Sorting functionality.
//

// no items defined
mod generic;
mod primitives;

mod definition; // Sort

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
