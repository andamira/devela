// devela::data::list::stack
//
#![doc = crate::_DOC_DATA_LIST_STACK!()] // public
#![doc = crate::_doc!(modules: crate::data::list; stack)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Elements are added and removed from the same end,
//! commonly referred to as the "top" of the stack.
//!
//! Stacks are ideal for managing nested or temporary operations.
//

mod adt;
#[cfg(_stack··)]
mod stack;

crate::structural_mods! { // _mods
    _mods {
        pub use super::adt::*;
        #[cfg(_stack··)]
        pub use super::stack::_all::*;
    }
}
