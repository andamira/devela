// devela::sys::mem::borrow
//
#![doc = crate::_DOC_SYS_MEM_BORROW!()]
//

mod backing; // Backing
mod maybe; // MaybeOwned
mod ownership; // Ownership, BackingChoice
mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{backing::*, maybe::*, ownership::*, reexports::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
