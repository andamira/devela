// devela::text
//
#![doc = crate::_DOC_TEXT!()]
#![doc = crate::_doc!(modules: crate; text: char, errors, fmt, grapheme, parse, str)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ascii, char, fmt, str, string)]
//
// safety
#![cfg_attr(feature = "safe_text", forbid(unsafe_code))]

pub mod errors {
    //! Text-related errors.
    #[doc(inline)]
    pub use devela_base_core::text::errors::*;
}

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod char;
pub mod fmt;
pub mod parse;
pub mod str;
// mod template; // WIP

#[cfg(feature = "grapheme")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "grapheme")))]
pub mod grapheme; // Grapheme

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            char::_all::*,
            errors::*,
            fmt::_all::*,
            parse::_all::*,
            str::_all::*,
            // template::*, // WIP
        };
        #[cfg(feature = "grapheme")]
        pub use super::{
            grapheme::_all::*,
        };
    }
}
