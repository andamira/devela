// devela::sys::mem::borrow
//
#![doc = crate::_DOC_SYS_MEM_BORROW!()]
//

mod backing; // Backing
mod maybe; // MaybeOwned
mod ownership; // Ownership, BackingChoice
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{backing::*, maybe::*, ownership::*, reexports::*};
    }
}
