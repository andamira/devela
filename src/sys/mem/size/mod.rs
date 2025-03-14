// devela::sys::mem::size
//
//! Memory size functionality.
//

mod byte; // ByteSized
mod expr;

#[cfg(feature = "bit")]
mod bit; // BitSized

crate::items! { // structural access: _mods, _hidden, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        #[doc(inline)]
        pub use super::{byte::*, expr::size_of_expr};

        #[doc(inline)]
        #[cfg(feature = "bit")]
        pub use super::bit::*;

        // WIPZONE
        // #[cfg(feature = "alloc")]
        // pub use super::heap::*;
    }
    pub(super) mod _hidden {
        #[doc(hidden)]
        pub use super::expr::__size_of_expr;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// #[cfg(feature = "alloc")]
// mod heap;
