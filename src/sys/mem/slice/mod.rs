// devela::sys::mem::slice
//
//! Slices of memory.
// #![doc = crate::doc_!(extends: slice)]
// #![doc = crate::doc_!(modules: crate::sys::mem; slice)]
// #![doc = crate::doc_!(newline)]

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
