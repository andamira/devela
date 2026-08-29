// devela/src/num/prob/stats/mod.rs
//
#![doc = crate::_DOC_NUM_PROB_STATS!()] // public
#![doc = crate::_doc!(modules: crate::num::prob; stats)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! > What can we infer about uncertainty from data?
//!
//! Statistical summaries describe observed data independently of how
//! that data was produced.
//!
//! Stateful accumulators favor streaming updates with bounded storage,
//! while derived measures expose interpretations of the retained state.
//

// mod estim; //
mod moment; // StatsMoment

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // estim::_all::*,
            moment::StatsMoment,
        };
    }
}
