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

#[cfg(feature = "_str_nonul")]
mod nonul;
#[cfg(feature = "_str_u8")]
mod string_u8;
#[cfg(feature = "alloc")]
mod string;

crate::structural_mods! { // _mods
    _mods {
        pub use super::r#trait::*;

        #[cfg(feature = "_str_nonul")]
        pub use super::nonul::*;
        #[cfg(feature = "_str_u8")]
        pub use super::string_u8::*;
        #[cfg(feature = "alloc")]
        pub use super::string::*;
    }
}
