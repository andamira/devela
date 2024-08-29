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
#[allow(unused_imports)]
pub use r#trait::*;

#[cfg(feature = "_string_nonul")]
mod nonul;
#[cfg(feature = "_string_u8")]
mod string_u8;

#[cfg(feature = "alloc")]
mod string;

#[allow(unused_imports)]
pub use all::*;
pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::r#trait::*;

    #[doc(inline)]
    #[cfg(feature = "_string_nonul")]
    pub use super::nonul::*;
    #[doc(inline)]
    #[cfg(feature = "_string_u8")]
    pub use super::string_u8::*;

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::string::*;
}
