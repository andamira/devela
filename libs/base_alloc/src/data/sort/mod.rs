// devela_base_alloc::data::sort
//
//! Sorting functionality.
//

mod definition; // SortAlloc
mod generic;

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
