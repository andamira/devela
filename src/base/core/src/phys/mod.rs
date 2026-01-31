// devela_base_core::phys
//
#![doc = crate::_DOC_PHYS!()] // public, root
#![doc = crate::_DOC_PHYS_MODULES!()]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(base_safe_phys, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_PHYS_MODULES =
    crate::_doc!(modules: crate; phys: time, wave); // atro, bio, chem, elec, geo, mech, time, unit
}

// pub mod astro;
// pub mod bio;
// pub mod chem;
// pub mod elec;
// pub mod geo;
// pub mod mech;
pub mod time;
// pub mod unit;
// pub mod wave; // Freq, Phase

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
            // unit::_all::*,
            // wave::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_PHYS_MODULES;
    }
}
