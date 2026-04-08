// devela_base_core::sys::mem::size
//
//! Memory size functionality.
//

mod byte; // ByteSized

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[doc(inline)]
        pub use super::{
            byte::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use crate::Sized;
    }
}
