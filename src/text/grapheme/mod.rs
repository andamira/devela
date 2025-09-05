// devela::text::grapheme
//
//! Extended Grapheme Clusters.
//!
//! The text between extended grapheme cluster boundaries as
//! specified by [UAX #29, "Unicode Text Segmentation"][0].
//!
//! [0]: https://www.unicode.org/reports/tr29/
//

mod r#trait; // Grapheme

crate::structural_mods! { // _mods
    _mods {
        pub use devela_base::text::{GraphemeNonul, GraphemeU8};
        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::text::GraphemeString;

        pub use super::r#trait::*;
    }
}
