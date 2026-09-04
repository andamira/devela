// devela/src/num/prob/_.rs
//
#![doc = crate::_DOC_NUM_PROB!()] // public
#![doc = crate::_doc!(modules: crate::num; prob: dist, markov, rand, stats)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! > What is the structure of uncertainty?
//

crate::mods_in! {
    pub mod_ dist; // Probability distributions
        // mod_ estim; // Estimation & inference
        // mod_ fit; // Model fitting
    pub mod_ markov; // Stochastic matrices, chains, traces
        mod probability; // Probability
    pub mod_ rand; // Random number generation
    pub mod_ stats; // Descriptive statistics
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            // estim::_all::*,
            // fit::_all::*,
            probability::Probability,
        };
    }
    _pub_mods {
        pub use super::{
            dist::_all::*,
            markov::_all::*,
            rand::_all::*,
            stats::_all::*,
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
