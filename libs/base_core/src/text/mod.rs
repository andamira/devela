// devela_base_core::text
//
#![doc = crate::_DOC_TEXT!()]
#![doc = crate::_DOC_TEXT_MODULES!()]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ascii, char, fmt, str, string)]
//
// safety
#![cfg_attr(base_safe_text, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_TEXT_MODULES =
    crate::_doc!(modules: crate; text: char, errors, fmt, parse, str); // grapheme
}

// mod cell; // TextCell, TextCellGrid WIP
// mod draw; // TextDraw WIP

mod lut; // TextLut

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod char; // Char[Ascii|Iter], Digits, UnicodeScalar, char[7|8|16|utf8]
pub mod errors; // Invalid[Char|Text|Utf8], TextError, TextResult
// pub mod grapheme; // Grapheme[Boundary|Machine|Scanner|U8|…] (in [base_text])
pub mod fmt; // DebugWith, FmtNum*, FmtWriter, fmtcat!, format_buf!
pub mod layout; // TextLayout*, …
pub mod parse; // ByteSearch
pub mod str; // Str, StringNonul, StringU*

crate::structural_mods! { // mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // cell::_all::*,
            // draw::_all::*,
            lut::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            char::_all::*,
            fmt::_all::*,
            errors::*,
            layout::_all::*,
            parse::_all::*,
            str::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_TEXT_MODULES;
    }
}
