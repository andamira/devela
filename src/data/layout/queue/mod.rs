// devela/src/data/layout/queue/mod.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_QUEUE!()] // private
#![doc = crate::_doc!(modules: crate::data::layout; queue)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Elements are typically added at one end (the "tail" or "back")
//! and removed from the other (the "head" or "front").
//!
//! Variants like double-ended queues (deques) allow insertion and removal
//! at both ends, providing additional flexibility.
//

#[cfg(feature = "alloc")]
mod _reexport_alloc;

mod adt;
// mod destaque; // WIP destaque!

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[doc(inline)]
        pub use super::{
            adt::*,
            // destaque::_all::*,
        };
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
    }
}
