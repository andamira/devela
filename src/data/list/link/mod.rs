// devela::data::list::link
//
//! Linked lists.
//!
//! Linked lists are sequentially accessed, homogeneous data structures.
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
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{r#const::*, reexports::*};
    }
}
