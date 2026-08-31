// devela/src/data/layout/stack/_.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_STACK!()] // private
#![doc = crate::_doc!(modules: crate::data::layout; stack)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Elements are added and removed from the same end,
//! commonly referred to as the "top" of the stack.
//!
//! Stacks are ideal for managing nested or temporary operations.
//

crate::mods_in! {
    // #[cfg(test)]
    // mod _test;

    mod adt;
    // mod define; // stack!
    // mod impls;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            adt::*,
            // define::*,
        };
    }
}
