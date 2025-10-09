// devela::text::grapheme
//
//! Extended Grapheme Clusters.
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

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            grapheme::*,
            kind::*,
        };

        #[doc(inline)]
        pub use devela_base_text::{
            GraphemeNonul, GraphemeU8, GraphemeScanner,
            GraphemeBoundary, GraphemeMachine, GraphemePropCb, GraphemePropInCb, GraphemeProps,
        };
        #[doc(inline)]
        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::text::GraphemeString;
    }
}
