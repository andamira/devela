// devela/src/phys/wave/_.rs
//
#![doc = crate::_DOC_PHYS_WAVE!()] // public
#![doc = crate::_doc!(modules: crate::phys; wave)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod shared; // WaveletHaar, WaveletUnitRole

    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    mod alloc;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::shared::*;

        #[cfg(feature = "alloc")]
        pub use super::alloc::*;
    }
}
