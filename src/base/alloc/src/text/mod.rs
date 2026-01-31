// devela_base_alloc::text
//
#![doc = crate::_DOC_TEXT!()] // public, root
#![doc = crate::_DOC_TEXT_MODULES!()]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: ascii, char, fmt, str, string)]
//
// safety
#![cfg_attr(base_safe_text, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_TEXT_MODULES =
    crate::_doc!(modules: crate; text: fmt, grapheme, str); // char, errors, parse
}

pub mod fmt;
pub mod grapheme; // GraphemeString
pub mod str;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            fmt::_all::*,
            grapheme::GraphemeString,
            str::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_TEXT_MODULES;
    }
}
