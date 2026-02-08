// devela_base_core::lang::hum
//
#![doc = crate::_DOC_LANG_HUM!()] // public
#![doc = crate::_doc!(modules: crate::lang; hum: art, i18n, nat)]
// denote, form, prag, syntax, vocal, write
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//! Structure and use of language as produced, interpreted, and shared by humans.
//

mod denote;
mod form;
mod prag;
mod syntax;
mod vocal;
mod write;

pub mod art; // artificial languages
pub mod i18n; // localization, translation
pub mod nat; // natural languages

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            denote::_all::*,
            form::_all::*,
            prag::_all::*,
            syntax::_all::*,
            vocal::_all::*,
            write::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            art::_all::*,
            i18n::_all::*,
            nat::_all::*,
        };
    }
}
