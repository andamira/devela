// devela/src/sys/os/term/event/input/mod.rs
//
//! Terminal input parsing.
//

#[cfg(test)]
mod tests;

mod parser; // TermInputParser

// internal
mod state; // (TermInputState, TermParsed, TermParsedCsi, TermReply)
mod csi; // impl parse control and csi methods for keys and mouse
mod feed; // impl feed_* methods

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            parser::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            state::*,
        };
    }
}
