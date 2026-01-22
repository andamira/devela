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
    crate::_doc!(modules: crate; text: char, errors, fmt, grapheme, parse, str);
}

// mod cell; // TextCell, TextCellGrid WIP
// mod draw; // TextDraw WIP

mod lut; // TextLut

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod char; // Char[Ascii|Iter], Digits, UnicodeScalar, char[7|8|16|utf8], transliterate
pub mod errors; // Invalid[Char|Text|Utf8], TextError, TextResult
#[cfg(feature = "grapheme")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "grapheme")))]
pub mod grapheme; // Grapheme[Nonul|U8], Grapheme[Boundary|Machine|Prop[Cb|InCb|s]|Scanner]
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
        #[cfg(feature = "grapheme")]
        pub use super::grapheme::*;
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_TEXT_MODULES,
            char::_workspace_internals::*,
        };
    }
}
