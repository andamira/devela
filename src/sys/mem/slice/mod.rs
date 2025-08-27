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
mod namespace; // Slice

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{ext::*, join::*, namespace::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
