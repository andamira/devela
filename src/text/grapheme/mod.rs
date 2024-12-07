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

#[cfg(feature = "_string_nonul")]
mod nonul;
#[cfg(feature = "_string_u8")]
mod string_u8;
#[cfg(feature = "alloc")]
mod string;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::r#trait::*;
        #[cfg(feature = "_string_nonul")]
        pub use super::nonul::*;
        #[cfg(feature = "_string_u8")]
        pub use super::string_u8::*;
        #[cfg(feature = "alloc")]
        pub use super::string::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
