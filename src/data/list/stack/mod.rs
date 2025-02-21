// devela::data::list::stack
//
//! Homogeneous data structures that process elements in
//! <abbr title="Last-In, First-Out">LIFO</abbr> order.
//!
//! Elements are added and removed from the same end,
//! commonly referred to as the "top" of the stack.
//!
//! Stacks are ideal for managing nested or temporary operations.
//

mod adt;
#[cfg(_stack··)]
mod stack;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::adt::*;
        #[cfg(_stack··)]
        pub use super::stack::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
