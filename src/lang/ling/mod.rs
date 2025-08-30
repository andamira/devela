// devela::lang::ling
//
//! Linguistics and languge theory.
#![doc = crate::_doc!(modules: crate::lang; ling: art, grammar, nat)]
//!
//! Structure, rules, analysis, and representation of natural and constructed languages.
//

pub mod art;
pub mod grammar;
pub mod nat;

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
            grammar::_all::*, art::_all::*, nat::_all::*,
        };
    }
}
