// devela::text
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
    crate::_doc!(modules: crate; text: char, error, fmt, grapheme, layout, parse, str);
}

// mod cell; // WIP Textel, TextelGrid
#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod char; // Char[Ascii|Iter], Digits, UnicodeScalar, char[7|8|16|utf8], transliterate
// mod draw; // WIP Drawing with unicode.
pub mod error; // Invalid[Char|Text|Utf8], TextError, TextResult
pub mod fmt; // DebugWith, FmtNum*, FmtWriter, fmtcat!, format_buf!
#[cfg(feature = "grapheme")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "grapheme")))]
pub mod grapheme; // Grapheme[Nonul|U8], Grapheme[Boundary|Machine|Prop[Cb|InCb|s]|Scanner]
// mod generate; // RENAME:WIP Procedures that produce text.
pub mod layout; // TextLayout*, …
mod lut; // TextLut
pub mod parse; // ByteSearch, TextScanner, …
pub mod str; // Str, StringNonul, StringU*
// mod template; // WIP
mod unit; // WIP TextUnit, TextIndex, TextCursor, TextRange

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // cell::_all::*, // WIP
            // draw::_all::*, // WIP
            // generate::*, // WIP
            lut::*,
            unit::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            char::_all::*,
            error::*,
            fmt::_all::*,
            layout::_all::*,
            parse::_all::*,
            str::_all::*,
            // template::*, // WIP
        };
        #[cfg(feature = "grapheme")]
        pub use super::{
            grapheme::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_TEXT_MODULES,
            char::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::fmt::_hidden::*;
    }
}
