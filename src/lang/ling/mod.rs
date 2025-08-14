// devela::lang::ling
//
//! Linguistics and languge theory.
#![doc = crate::doc_!(modules: crate::lang; ling: art, grammar, nat)]
//!
//! Structure, rules, analysis, and representation of natural and constructed languages.
//

pub mod art;
pub mod grammar;
pub mod nat;

crate::items! { // structural access: _pub_mods, _mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods { #![allow(unused)]
        // pub use super::grammar::_all::*;
        // pub use super::art::_all::*;
        // pub use super::nat::_all::*;
    }
    mod _mods { #![allow(unused)]
        // WIPZONE:
        // pub use super::phonetics::_all::*;
        // pub use super::prosody::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_pub_mods::*, _mods::*};
    }
}
// WIPZONE
// mod phonetics;
// mod prosody;
