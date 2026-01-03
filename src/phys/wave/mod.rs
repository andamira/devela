// devela::phys::wave
//
//! Wavelets.
//

#[cfg(test)]
mod tests;

mod shared; // WaveletHaar, WaveletUnitRole

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod alloc;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::shared::*;

        #[cfg(feature = "alloc")]
        pub use super::alloc::*;
    }
    _reexports {
    }
}
