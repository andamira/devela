// devela_base_text::text
//
#![doc = crate::_DOC_TEXT!()]
//

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
mod char; // transliterate
pub mod grapheme; // Grapheme[Nonul|U8], Grapheme[Boundary|Machine|Prop[Cb|InCb|s]|Scanner]

crate::structural_mods! { // _mods, _workspace_internals
    _mods {
        pub use super::{
            char::_all::*,
            grapheme::_all::*,
        };
    }
    _workspace_internals {
        pub use super::{
            char::_workspace_internals::*,
        };
    }
}
