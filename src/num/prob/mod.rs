// devela::num::prob
//
#![doc = crate::_DOC_NUM_PROB!()] // public
#![doc = crate::_doc!(modules: crate::num; prob: rand)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! > What is the structure of uncertainty?
//

// mod dist; // Probability distributions
// mod estim; // Estimation & inference
// mod fit; // Model fitting

pub mod rand; // Random number generation
// pub mod stats; // Descriptive statistics

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        // pub use super::{
        //     dist::_all::*,
        //     estim::_all::*,
        //     fit::_all::*,
        // };
    }
    _pub_mods {
        pub use super::{
            rand::_all::*,
            // stats::_all::*,
        };
    }
}
