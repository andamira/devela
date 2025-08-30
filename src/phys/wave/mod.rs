// devela::phys::wave
//
//! Wavelets.
//

#[cfg(test)]
mod tests;

mod shared; // WaveletHaar, WaveletUnitRole

#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "_float_f64"))] // NOTE: not documented
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod alloc;

crate::structural_mods! { // _mods
    _mods {
        pub use super::shared::*;

        #[cfg(feature = "alloc")]
        #[cfg(any(feature = "std", feature = "_float_f64"))]
        pub use super::alloc::*;
    }
}
