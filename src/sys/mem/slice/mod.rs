// devela::sys::mem::slice
//
//! Slices of memory.
// #![doc = crate::_doc!(extends: slice)]
// #![doc = crate::_doc!(modules: crate::sys::mem; slice)]
// #![doc = crate::_doc!(newline)]

#[cfg(test)]
mod tests;

mod ext; // ExtSlice
mod join; // join!

crate::structural_mods! { // _mods
    _mods {
        pub use devela_base::sys::mem::Slice;

        pub use super::{ext::*, join::*};
    }
}
