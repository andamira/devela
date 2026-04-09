// devela::text::grapheme
//
#![doc = crate::_DOC_TEXT_GRAPHEME!()] // public
#![doc = crate::_doc!(modules: crate::text; grapheme)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]
//!
//! The text between extended grapheme cluster boundaries as
//! specified by [UAX #29, "Unicode Text Segmentation"][0].
//!
//! [0]: https://www.unicode.org/reports/tr29/
//

#[cfg(test)]
mod tests;

mod grapheme; // Grapheme
mod kind; // GraphemeKind
mod nonul; // GraphemeNonul
mod scanner; // Grapheme[Boundary|Machine|Prop[Cb|InCb|s]|Scanner]
mod u8; // GraphemeU8

#[cfg(feature = "alloc")]
mod string; // GraphemeString

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            grapheme::*,
            kind::*,
            nonul::*,
            scanner::_all::*,
            u8::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::string::GraphemeString;
    }
}
