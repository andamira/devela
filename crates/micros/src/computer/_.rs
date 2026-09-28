//
#![doc = crate::_DOC_COMPUTER!()] // public
#![doc = crate::_doc!(modules: crate; computer: zx)] // amstrad, msx, nintendo, sega
#![doc = crate::_doc!(flat:"computer")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // #[cfg_attr(not(nightly_doc), cfg(feature = "amstrad"))]
    // pub mod_ amstrad;
    // #[cfg_attr(not(nightly_doc), cfg(feature = "msx"))]
    // pub mod_ msx;
    // #[cfg_attr(not(nightly_doc), cfg(feature = "nintendo"))]
    // pub mod_ nintendo;
    // #[cfg_attr(not(nightly_doc), cfg(feature = "sega"))]
    // pub mod_ sega;
    #[cfg_attr(not(nightly_doc), cfg(feature = "zx"))]
    pub mod_ zx;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        // #[cfg_attr(not(nightly_doc), cfg(feature = "amstrad"))]
        // pub use super::amstrad::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "msx"))]
        // pub use super::msx::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "nintendo"))]
        // pub use super::nintendo::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "sega"))]
        // pub use super::sega::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "zx"))]
        pub use super::zx::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "spectrum48")]
        pub use super::zx::ComputerSpectrum48;
    }
}
