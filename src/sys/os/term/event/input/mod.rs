// devela::sys::os::term::event::input

#[cfg(test)]
mod tests;

mod _helper; // (TermInputState, TermParsed, TermParsedCsi, TermReply), …
mod parser; // TermInputParser

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            parser::*,
        };
    }
}
