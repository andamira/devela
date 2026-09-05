// devela/src/sys/os/term/event/input/_.rs
//
//! Terminal input parsing.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod parser; // TermInputParser

    // internal
    mod state; // (TermInputState, TermParsed, TermParsedCsi, TermReply)
    mod csi; // impl parse control and csi methods for keys and mouse
    mod feed; // impl feed_* methods
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            parser::TermInputParser,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            state::*,
        };
    }
}
