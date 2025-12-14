// devela_base_core::sys::mem::size
//
//! Memory size functionality.
//

mod byte; // ByteSized
mod expr; // size_of_expr!

crate::structural_mods! { // _mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            byte::*,
            expr::size_of_expr,
        };
    }
    _hidden {
        pub use super::expr::__size_of_expr;
    }
}
