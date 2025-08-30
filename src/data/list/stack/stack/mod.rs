// devela::data::list::stack::stack
//
//! A type that can be used as a single-ended stack.
//

// no items defined
mod impl_traits;
mod methods;

mod definitions; // Stack, StackIter, …

crate::structural_mods! { // _mods
    _mods {
        pub use super::definitions::*;
    }
}
