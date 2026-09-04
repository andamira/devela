// devela/src/num/prob/stats/_.rs
//
#![doc = crate::_DOC_NUM_PROB_STATS!()] // public
#![doc = crate::_doc!(modules: crate::num::prob; stats)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! > How can observed data be summarized?
//!
//! Statistical summaries describe observed data independently of how
//! that data was produced.
//!
//! Stateful accumulators favor streaming updates with bounded storage,
//! while derived measures expose interpretations of the retained state.
//

crate::mods_in! {
    mod moment; // StatsMoment
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            moment::StatsMoment,
        };
    }
}
