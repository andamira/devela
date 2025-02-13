// devela::phys::wave
//
//! Wavelets.
//

#[cfg(test)]
mod tests;

mod shared; // WaveletHaar, WaveletUnitRole

#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "_float_f64"))] // NOTE: not documented
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod alloc;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::shared::*;

        #[cfg(feature = "alloc")]
        #[cfg(any(feature = "std", feature = "_float_f64"))]
        pub use super::alloc::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
