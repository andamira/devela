// devela/src/num/prob/rand/prng/_.rs
//
#![doc = crate::_DOC_NUM_PROB_RAND_PRNG!()] // private
#![doc = crate::_doc!(modules: crate::num::prob::rand; prng)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // #[cfg(feature = "rand")]
    // mod_ chacha; // TODO
    #[cfg(feature = "rand")]
    mod_ lcg; // Lcg16
    mod_ pcg; // Pcg[8|16|32|64|128], rand_pcg!
    #[cfg(feature = "rand")]
    mod_ shift; // XorShift[8|16|32|64|128], XorShift128p, (rand_xorshift!)
    mod splitmix; // SplitMix64

    #[cfg(feature = "rand")]
    mod_ xoroshiro; // Xoroshiro128pp
    #[cfg(feature = "rand")]
    mod xyza8; // Xyza8a, Xyza8b
    #[cfg(feature = "rand")]
    mod xabc; // Xabc
}
crate::mods_out! { // _mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            pcg::_all::*,
            splitmix::*,
        };
        #[cfg(feature = "rand")]
        pub use super::{
            // chacha::_all::*,
            lcg::_all::Lcg16,
            shift::_all::*,
            xabc::Xabc,
            xoroshiro::_all::Xoroshiro128pp,
            xyza8::{Xyza8a, Xyza8b},
        };
    }
    _crate_internals {
        #[cfg(feature = "rand")]
        pub(crate) use super::shift::_crate_internals::*;
    }
    _hidden {
        #[cfg(feature = "rand")]
        pub(crate) use super::shift::_hidden::*;
    }
}
