// devela/src/sys/mem/size/mod.rs
//
//! Memory size functionality.
//

mod bit; // BitSized
mod byte; // ByteSized
mod expr; // size_of_expr!
// #[cfg(feature = "alloc")]
// mod heap; // WIP

crate::structural_mods! { // _mods, _reexports, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            bit::*,
            byte::*,
            expr::size_of_expr,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use crate::Sized;
    }
    _hidden {
        pub use super::expr::__size_of_expr;
    }
}
