// devela::sys::mem::slice
//
//! Slices of memory.
// #![doc = crate::_doc!(extends: slice)]
// #![doc = crate::_doc!(modules: crate::sys::mem; slice)]
// #![doc = crate::_doc!(newline)]

#[cfg(test)]
mod tests;

mod ext; // ExtSlice

crate::structural_mods! { // _mods
    _mods {
        pub use super::ext::*;

        // re-exports
        #[doc(inline)]
        pub use devela_base::sys::mem::{Slice, const_join};
    }
}
