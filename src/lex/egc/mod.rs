// devela::lex::egc
//
//! Extended Grapheme Clusters.
//!
//! The text between extended grapheme cluster boundaries as
//! specified by [UAX #29, "Unicode Text Segmentation"][0].
//!
//! [0]: https://www.unicode.org/reports/tr29/
//

mod r#trait; // Egc
#[allow(unused_imports)]
pub use r#trait::*;

#[cfg(feature = "_string_nonul")]
mod nonul;
#[cfg(all(feature = "lex", feature = "alloc"))]
mod string;
#[cfg(feature = "_string_u8")]
mod string_u8;

#[allow(unused_imports)]
pub use all::*;
pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::r#trait::*;

    #[doc(inline)]
    #[cfg(feature = "_string_u8")]
    pub use super::string_u8::*;

    #[doc(inline)]
    #[cfg(feature = "_string_nonul")]
    pub use super::nonul::*;

    #[doc(inline)]
    #[cfg(all(feature = "lex", feature = "alloc"))]
    pub use super::string::*;
}
