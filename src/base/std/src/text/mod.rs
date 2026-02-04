// devela_base_std::text
//
#![doc = crate::_DOC_TEXT!()] // public, root
#![doc = crate::_DOC_TEXT_MODULES!()]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: ascii, char, fmt, str, string)]
//
// safety
#![cfg_attr(feature = "safe_text", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_TEXT_MODULES =
    crate::_doc!(modules: crate; text: str); // char, errors, fmt, grapheme, parse
}

pub mod str;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            // fmt::_all::*,
            // parse::_all::*,
            str::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_TEXT_MODULES;
    }
}
