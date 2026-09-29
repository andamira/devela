//
//!
//

crate::mods_in! {
    mod attribute;
    mod color;
    mod key;
    mod ula;

    // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum16"))]
    // mod_ s16;
    #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum48"))]
    mod_ s48;
    // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum128"))]
    // mod_ s128;
    // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum_next"))]
    // mod_ next;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            attribute::SpectrumAttribute,
            color::SpectrumColor,
            key::{SpectrumKey, SpectrumKeys},
            ula::SpectrumUlaOut,
        };
        // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum16"))]
        // pub use super::s16::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum48"))]
        pub use super::s48::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum128"))]
        // pub use super::s128::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "spectrum_next"))]
        // pub use super::next::_all::*;
    }
}
