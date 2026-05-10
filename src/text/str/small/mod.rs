// devela::text::str::small
//
//! Defines strings with small-string optimizations.
//

#[cfg(feature = "alloc")]
mod alloc; // StringSmallAlloc

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "alloc")]
        pub use super::alloc::*;
    }
}
