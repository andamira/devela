// devela_base_core::num::prob::rand::prng
//
#![doc = crate::_DOC_NUM_PROB_RAND_PRNG!()] // private
#![doc = crate::_doc!(modules: crate::num::prob::rand; prng)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod shift; // XorShift[8|16|32|64|128], XorShift128p, xorshift_custom!
mod pcg; // Pcg[8|16|32|64|128], define_pcg!

#[cfg(feature = "rand")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod lcg; // Lcg16
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod xoroshiro; // Xoroshiro128pp
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod xyza8; // Xyza8a, Xyza8b
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod xabc; // Xabc
}

crate::structural_mods! { // _mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            pcg::_all::*,
            shift::_all::*,
        };

        #[cfg(feature = "rand")]
        pub use super::{
            lcg::_all::*,
            xabc::*,
            xoroshiro::*,
            xyza8::*,
        };
    }
    _crate_internals {
        #[cfg(feature = "rand")]
        pub(crate) use super::shift::_crate_internals::*;
    }
    _hidden {
        pub(crate) use super::shift::_hidden::*;
    }
}
