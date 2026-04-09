// devela::phys::wave
//
#![doc = crate::_DOC_PHYS_WAVE!()] // public
#![doc = crate::_doc!(modules: crate::phys; wave)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests;

mod shared; // WaveletHaar, WaveletUnitRole

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod alloc;

crate::structural_mods! { // _mods
    _mods {
        pub use super::shared::*;

        #[cfg(feature = "alloc")]
        pub use super::alloc::*;
    }
}
