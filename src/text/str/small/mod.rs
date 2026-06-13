// devela/src/text/str/small/mod.rs
//
//! Inline-first UTF-8 string storage with spillover.
//

#[cfg(feature = "alloc")]
mod alloc; // StringSmallAlloc

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "alloc")]
        pub use super::alloc::*;
    }
}
