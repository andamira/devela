// devela::text::unicode
//
#![doc = crate::_DOC_TEXT_UNICODE!()] // public
#![doc = crate::_doc!(modules: crate::text; unicode: grapheme, scalar)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: char)]
//

// mod case;
#[cfg(feature = "grapheme")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "grapheme")))]
pub mod grapheme; // Grapheme[Nonul|U8], Grapheme[Boundary|Machine|Prop[Cb|InCb|s]|Scanner]
// mod norm;
// mod prop; // Unicode properties
// mod width;
#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod scalar; // Char[Ascii|Iter], Digits, UnicodeScalar, char[7|8|16|utf8]

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        // pub use super::{
        //     // norm::*,
        //     // prop::*,
        //     // width::*,
        // };
    }
    _pub_mods {
        pub use super::{
            scalar::_all::*,
        };
        #[cfg(feature = "grapheme")]
        pub use super::{
            grapheme::_all::*,
        };

    }
}
