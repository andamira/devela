// devela/src/num/prob/dist/mod.rs
//
#![doc = crate::_DOC_NUM_PROB_DIST!()] // public
#![doc = crate::_doc!(modules: crate::num::prob; dist)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! Probability laws represented independently of random sources.
//!
//! Distributions describe probabilistic structure.
//! Sampling combines that structure with a random source.
//!
//! The foundational representations favor exact, allocation-free,
//! const-compatible forms with explicit preparation and sampling costs.
//

mod binomial; // DistBernoulli, DistBinomial
mod categorical; // DistCategorical
// mod continuous;
// mod sample;
// mod view;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            binomial::{DistBernoulli, DistBinomial},
            categorical::DistCategorical,
            // continuous::*,
            // sample::*,
            // view::*,
        };
    }
    _reexports {
        pub use crate::DistError;
    }
}
