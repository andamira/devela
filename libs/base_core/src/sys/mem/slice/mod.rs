// devela_base_core::sys::mem::slice
//
//! Slices of memory.
//

mod join; // const_join!
mod namespace; // Slice, slice!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            namespace::{Slice, slice},
            join::const_join,
        };
    }
}
