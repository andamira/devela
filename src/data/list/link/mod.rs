// devela::data::list::link
//
#![doc = crate::_DOC_DATA_LIST_LINK!()]
//!
//! They enable efficient insertion and deletion at any position,
//! storing a sequence of elements of the same type, each pointing to the next.
//

mod r#const;
mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{r#const::*, reexports::*};
        // WIPZONE
        // pub use super::l1::*;
        // pub use super::l2::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{r#const::*, reexports::*};
    }
}
// WIPZONE
// #[cfg(_list1··)]
// mod l1;
// #[cfg(_list2··)]
// mod l2;
