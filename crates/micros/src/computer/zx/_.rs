//
//!
#![doc = crate::_DOC_COMPUTER_ZX!()] // public
#![doc = crate::_doc!(modules: crate::computer; zx: spectrum)] //
#![doc = crate::_doc!(flat:"computer")]
#![doc = crate::_doc!(hr)]
//!
//! Current support is for the ZX Spectrum family through [`spectrum`].
//

crate::mods_in! {
    // mod_ zx80;
    // mod_ zx81;
    #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum"))]
    pub mod_ spectrum;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum"))]
        pub use super::spectrum::_all::*;
    }
    _reexports {
        #[doc(inline)] #[cfg(feature = "spectrum48")]
        pub use super::spectrum::ComputerSpectrum48;
    }
}
