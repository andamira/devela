//
#![doc = crate::_DOC_DATA_LAYOUT_LINKED!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; linked)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Linked layouts represent sequence order through explicit element relations.
//!
//! The links establish succession independently of how elements are owned or stored.
//

crate::mods_in! {
    mod r#const; // ConstList[Iterator]

    // #[cfg(_list1··)]
    // mod l1;
    // #[cfg(_list2··)]
    // mod l2;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::r#const::*;
        // pub use super::l1::*;
        // pub use super::l2::*;
    }
    _reexports {
        pub use crate::boot::rust::data::layout_linked::*;
    }
}
