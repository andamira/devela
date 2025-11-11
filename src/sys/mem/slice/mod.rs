// devela::sys::mem::slice
//
//! Slices of memory.
// #![doc = crate::_doc!(extends: slice)]
// #![doc = crate::_doc!(modules: crate::sys::mem; slice)]
// #![doc = crate::_doc!(newline)]

#[cfg(test)]
mod tests;

mod ext; // SliceExt

crate::structural_mods! { // _mods
    _mods {
        pub use super::ext::*;

        // re-exports
        #[doc(inline)]
        pub use devela_base_core::sys::mem::{
            Slice, const_join, slice,
        };
    }
}
