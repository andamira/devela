// devela::sys::mem::size
//
//! Memory size functionality.
//

#[cfg(feature = "bit")]
mod bit; // BitSized
mod expr; // size_of_expr!
// #[cfg(feature = "alloc")]
// mod heap; // WIP

crate::structural_mods! { // _mods, _reexports, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            expr::size_of_expr,
        };
        #[doc(inline)]
        #[cfg(feature = "bit")]
        pub use super::bit::*;
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::sys::mem::{
            ByteSized,
        };
        #[doc(inline)]
        pub use crate::Sized;
    }
    _hidden {
        pub use super::expr::__size_of_expr;
    }
}
