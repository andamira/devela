// devela::sys::mem::size
//
//! Memory size functionality.
//

#[cfg(feature = "bit")]
mod bit; // BitSized

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        #[cfg(feature = "bit")]
        pub use super::bit::*;

        // re-exports
        #[doc(inline)]
        pub use devela_base_core::sys::mem::{
            ByteSized, size_of_expr
        };
    }
    _hidden {
        // re-exports
        pub use devela_base_core::sys::mem::__size_of_expr;
    }
}
