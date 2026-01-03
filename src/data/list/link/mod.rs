// devela::data::list::link
//
#![doc = crate::_DOC_DATA_LIST_LINK!()]
//!
//! They enable efficient insertion and deletion at any position,
//! storing a sequence of elements of the same type, each pointing to the next.
//

#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /libs/base_alloc/src/data/list/link/_reexport.rs

// #[cfg(_list1··)]
// mod l1;
// #[cfg(_list2··)]
// mod l2;

crate::structural_mods! { // _mods, _reexports
    _mods {
        // pub use super::l1::*;
        // pub use super::l2::*;
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        pub use devela_base_core::data::list::{
            ConstList, ConstListIterator,
        };
    }
}
