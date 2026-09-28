//
//!
// #![doc = crate::_DOC_COMPUTER_ZX!()] // public
// #![doc = crate::_doc!(modules: crate::computer; zx)] //
// #![doc = crate::_doc!(flat:"computer")]
// #![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ zx80;
    // mod_ zx81;
    #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum"))]
    mod_ spectrum;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum"))]
        pub use super::spectrum::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "spectrum48")]
        pub use super::spectrum::ComputerSpectrum48;
    }
}
