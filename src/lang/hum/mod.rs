// devela::lang::hum
//
//! Human linguistics and language theory.
#![doc = crate::_doc!(modules: crate::lang; hum: art, gram, i18n, nat)]
//!
//! Structure, rules, analysis, and representation of natural and constructed languages.
//

pub mod art; // artificial languages
pub mod gram; // grammar
pub mod i18n; // gettext, fluent, …
pub mod nat; // natural languages

// WIPZONE
// mod phonetics;
// mod prosody;

crate::structural_mods! { // _pub_mods, _mods
    _mods {
        // WIPZONE:
        // pub use super::phonetics::_all::*;
        // pub use super::prosody::_all::*;
    }
    _pub_mods {
        pub use super::{
            art::_all::*,
            gram::_all::*,
            i18n::_all::*,
            nat::_all::*,
        };
    }
}
