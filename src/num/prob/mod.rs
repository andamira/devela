// devela/src/num/prob/mod.rs
//
#![doc = crate::_DOC_NUM_PROB!()] // public
#![doc = crate::_doc!(modules: crate::num; prob: dist, rand, stats)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! > What is the structure of uncertainty?
//

pub mod dist; // Probability distributions
// mod estim; // Estimation & inference
// mod fit; // Model fitting
// mod markov; // Stochastic matrices, chains, traces
mod probability; // Probability
pub mod rand; // Random number generation
pub mod stats; // Descriptive statistics

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
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
            // markov::_all::*,
            rand::_all::*,
            stats::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::rand::_crate_internals::*;
    }
    _hidden {
        pub use super::rand::_hidden::*;
    }
}
