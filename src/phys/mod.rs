// devela::phys
//
#![doc = crate::_DOC_PHYS!()]
#![doc = crate::_doc!(modules: crate; phys: bio, chem, elec, mech, time, unit, wave)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_phys", forbid(unsafe_code))]

pub mod bio;
pub mod chem;
pub mod elec;
pub mod mech;
pub mod time;
pub mod unit;

#[cfg(feature = "wave")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "wave")))]
pub mod wave;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            bio::_all::*, chem::_all::*, elec::_all::*,
            time::_all::*, mech::_all::*, unit::_all::*,
        };
        #[cfg(feature = "wave")]
        pub use super::wave::_all::*;
    }
}
