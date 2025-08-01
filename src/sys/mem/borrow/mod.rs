// devela::sys::mem::borrow
//
//! Borrowed data.
//

mod alloc; // AllocMode
mod maybe; // MaybeOwned
mod ownership; // Ownership, BackingChoice
mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{alloc::*, maybe::*, ownership::*, reexports::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
