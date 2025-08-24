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

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{adt::*, reexports::*};

        #[cfg(_destaque··)]
        pub use super::destaque::_all::*;
        // WIPZONE
        // pub use super::destaque_define::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{adt::*, reexports::*};
    }
}
// WIPZONE
// mod destaque_define; // WIP:
