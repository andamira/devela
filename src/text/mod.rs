// devela/src/text/mod.rs
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
    crate::_doc!(modules: crate; text: ascii, error, fmt, layout, parse, str, unicode);
}

pub mod ascii; // AsciiSet, CharAscii
// mod draw; // WIP Drawing with text.
pub mod error; // Invalid[Char|Text|Utf8], TextError, TextResult
pub mod fmt; // DebugWith, FmtNum*, FmtWriter, fmtcat!, format_buf!
// mod generate; // WIP Procedures that produce text.
pub mod layout; // TextLayout*, …
mod metric; // TextCursor, TextIndex, TextRange, TextUnit
pub mod parse; // ByteSearch, TextScanner, …
pub mod str; // Str, StringNonul, StringU*
mod translit; // scalar_as_ascii_translit()
pub mod unicode; // Unicode-defined text units and algorithms

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // draw::_all::*,
            // generate::_all::*,
            metric::*,
            translit::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            ascii::*,
            error::*,
            fmt::_all::*,
            layout::_all::*,
            parse::_all::*,
            str::_all::*,
            unicode::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_TEXT_MODULES,
        };
    }
    _hidden {
        pub use super::fmt::_hidden::*;
    }
}
