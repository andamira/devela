//
#![doc = crate::_DOC_NUM_PROB!()] // public
#![doc = crate::_doc!(modules: crate::num; prob: dist, markov, rand, stats)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! > What is the structure of uncertainty?
//

crate::mods_in! {
    #[cfg(feature = "prob")]
    pub mod_ dist; // Probability distributions
        // mod_ estim; // Estimation & inference
        // mod_ fit; // Model fitting
    #[cfg(feature = "prob")]
    pub mod_ markov; // Stochastic matrices, chains, traces
        #[cfg(feature = "prob")]
        mod probability; // Probability
    pub mod_ rand; // Random number generation
    #[cfg(feature = "prob")]
    pub mod_ stats; // Descriptive statistics
}
crate::mods_out! { // _mods, _pub_mods, _reexports _crate_internals, _hidden
    _mods {
        #[cfg(feature = "prob")]
        pub use super::probability::Probability;
        // pub use super::{
        //     estim::_all::*,
        //     fit::_all::*,
        // };
    }
    _pub_mods {
        #[cfg(feature = "prob")]
        pub use super::{
            dist::_all::*,
            markov::_all::*,
            stats::_all::*,
        };
        pub use super::rand::_all::*;
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            rand::{Pcg32, RandTry},
        };
    }
    _crate_internals {
        pub(crate) use super::{
            rand::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            rand::_hidden::*,
        };
    }
}
