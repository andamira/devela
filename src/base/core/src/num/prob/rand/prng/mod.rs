// devela_base_core::num::prob::rand::prng
//
#![doc = crate::_DOC_NUM_PROB_RAND_PRNG!()]
//!
//

mod shift; // XorShift[8|16|32|64|128], XorShift128p, xorshift_custom!

#[cfg(feature = "rand")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod lgc; // Lgc16
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod xoroshiro; // Xoroshiro128pp
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod xyza8; // Xyza8a, Xyza8b
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    mod xabc; // Xabc
}

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::shift::*;

        #[cfg(feature = "rand")]
        pub use super::{
            lgc::*,
            xabc::*,
            xoroshiro::*,
            xyza8::*,
        };
    }
    _crate_internals {
        #[cfg(feature = "rand")]
        pub(crate) use super::shift::xorshift_basis;
    }
}
