// devela::data::list::stack::stack
//
//! A type that can be used as a single-ended stack.
//

// no items defined
mod impl_traits;
mod methods;

mod definitions; // Stack, StackIter, …

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::definitions::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
