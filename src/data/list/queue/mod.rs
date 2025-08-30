// devela::data::list::queue
//
#![doc = crate::_DOC_DATA_LIST_QUEUE!()]
//!
//! Elements are typically added at one end (the "tail" or "back")
//! and removed from the other (the "head" or "front").
//!
//! Variants like double-ended queues (deques) allow insertion and removal
//! at both ends, providing additional flexibility.
//

mod adt;
#[cfg(_destaque··)]
mod destaque;
mod reexports;

// WIPZONE
// mod destaque_define; // MAYBE

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{adt::*, reexports::*};

        #[cfg(_destaque··)]
        pub use super::destaque::_all::*;
        // WIPZONE
        // pub use super::destaque_define::_all::*;
    }
    _always {
        pub use super::{adt::*, reexports::*};
    }
}
