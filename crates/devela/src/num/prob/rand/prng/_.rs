//
#![doc = crate::_DOC_NUM_PROB_RAND_PRNG!()] // private
#![doc = crate::_doc!(modules: crate::num::prob::rand; prng)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // #[cfg(feature = "prng")]
    // mod_ chacha; // TODO
    #[cfg(feature = "prng")]
    mod_ lcg; // Lcg16
    mod_ pcg; // Pcg[8|16|32|64|128], rand_pcg!
    #[cfg(feature = "prng")]
    mod_ shift; // XorShift[8|16|32|64|128], XorShift128p, (rand_xorshift!)
    mod splitmix; // SplitMix64

    #[cfg(feature = "prng")]
    mod_ xoroshiro; // Xoroshiro128pp
    #[cfg(feature = "prng")]
    mod xyza8; // Xyza8a, Xyza8b
    #[cfg(feature = "prng")]
    mod xabc; // Xabc
}
crate::mods_out! { // _mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            pcg::_all::*,
            splitmix::*,
        };
        #[cfg(feature = "prng")]
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
        #[cfg(feature = "prng")]
        pub(crate) use super::shift::_crate_internals::*;
    }
    _hidden {
        #[cfg(feature = "prng")]
        pub(crate) use super::shift::_hidden::*;
    }
}
