// devela/src/code/util/token/mod.rs
//
//! Macro token, fragment, identifier, and literal utilities
//

mod _reexport_core;

mod capture; // capture_[first|last|tail_tuple]!
mod dollar; // macro_dollar!
mod ident; // ident_const_index!

crate::structural_mods! { // _mods, _reexports,
    _mods {
        #[doc(inline)]
        pub use super::{
            capture::{capture_first, capture_last, capture_tail_tuple},
            dollar::macro_dollar,
            ident::ident_const_index,
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
