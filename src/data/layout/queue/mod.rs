// devela::data::layout::queue
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
mod _reexport_alloc; // SYMLINK to /crates/base/alloc/src/data/layout/queue/_reexport.rs

mod adt;
#[cfg(_destaque··)]
mod destaque;
// mod define_destaque; // WIP

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::adt::*;

        #[cfg(_destaque··)]
        pub use super::destaque::_all::*;
        // pub use super::define_destaque::_all::*;
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
    }
}
