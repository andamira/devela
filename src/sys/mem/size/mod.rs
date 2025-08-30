// devela::sys::mem::size
//
//! Memory size functionality.
//

mod byte; // ByteSized
mod expr;

#[cfg(feature = "bit")]
mod bit; // BitSized

// WIPZONE
// #[cfg(feature = "alloc")]
// mod heap;

crate::structural_mods! { // _mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{byte::*, expr::size_of_expr};

        #[doc(inline)]
        #[cfg(feature = "bit")]
        pub use super::bit::*;

        // WIPZONE
        // #[cfg(feature = "alloc")]
        // pub use super::heap::*;
    }
    _hidden {
        #[doc(hidden)]
        pub use super::expr::__size_of_expr;
    }
}
