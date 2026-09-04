// devela/src/num/prob/markov/_.rs
//
#![doc = crate::_DOC_NUM_PROB_MARKOV!()] // public
#![doc = crate::_doc!(modules: crate::num::prob; markov)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! Markov structure describes transitions whose next-state distribution
//! depends only on the current state.
//!
//! A transition kernel associates each state with a probability law over
//! the states that may follow it.
//!
//! Chains, walks, traces, and convergence measures can be layered over
//! that transition structure.
//

crate::mods_in! {
    // mod analysis;
    // mod chain; // MarkovChain
    mod kernel; // MarkovKernel
    // mod trace; // MarkovTrace
    // mod walk; // MarkovWalk
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // analysis::_all::*,
            // chain::*,
            kernel::MarkovKernel,
            // trace::*,
            // walk::*,
        };
    }
}
