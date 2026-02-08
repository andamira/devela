// devela_base_core::text::grapheme
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

mod scanner; // Grapheme[Boundary|Machine|Prop[Cb|InCb|s]|Scanner]
mod nonul; // GraphemeNonul
mod u8; // GraphemeU8

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            scanner::_all::*,
            nonul::*,
            u8::*,
        };
    }
}
