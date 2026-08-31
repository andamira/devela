// devela/src/code/util/token/_.rs
//
#![doc = crate::_DOC_CODE_UTIL_TOKEN!()] // public
#![doc = crate::_doc!(modules: crate::code::util; token)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//!
//! Token utilities operate on macro input material itself: token trees,
//! fragments, identifiers, and literals. Construction and composition of
//! higher-level Rust structure belongs to [`synth`][super::synth].
//

crate::mods_in! {
    mod _reexport_core;

    mod capture; // capture_[first|last|tail_tuple]!
    mod dollar; // macro_dollar!
    mod ident; // ident_const_index!
    mod sf; // sf!
    mod type_count; // type_count!
}
crate::mods_out! { // _mods, _reexports,
    _mods {
        #[doc(inline)]
        pub use super::{
            capture::{capture_first, capture_last, capture_tail_tuple},
            dollar::macro_dollar,
            ident::ident_const_index,
            sf::sf,
            type_count::type_count,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc = crate::_tags!(code procedural_macro)]
        pub use devela_macros::{
            coalesce,
            ident_total, ident_total_unique, ident_unique,
            paste,
        };
    }
}
