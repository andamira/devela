// devela::phys
//
#![doc = crate::_DOC_PHYS!()] // public
#![doc = crate::_DOC_PHYS_MODULES!()]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_phys", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_PHYS_MODULES =
    crate::_doc!(modules: crate; phys: time, unit, wave); // astro, bio, chem, elec, geo, mech
}

// pub mod astro;
// pub mod bio;
// pub mod chem;
// pub mod elec;
// pub mod geo;
// pub mod mech;
pub mod time;

#[cfg(feature = "unit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unit")))]
pub mod unit;

#[cfg(feature = "wave")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "wave")))]
pub mod wave;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            // astro::_all::*,
            // bio::_all::*,
            // chem::_all::*,
            // elec::_all::*,
            // geo::_all::*,
            // mech::_all::*,
            time::_all::*,
        };
        #[cfg(feature = "unit")]
        pub use super::unit::_all::*;
        #[cfg(feature = "wave")]
        pub use super::wave::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_PHYS_MODULES;
    }
}
