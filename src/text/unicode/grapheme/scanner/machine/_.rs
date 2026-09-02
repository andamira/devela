// devela/src/text/unicode/grapheme/scanner/machine/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod_ _test;

    mod define;
    mod state;
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            define::{GraphemeMachine, GraphemeBoundary},
        };
    }
    // _self_internals {} // TODO
}
use state::GraphemeMachineState; // IMPROVE
