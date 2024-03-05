// devela::text::egc
//
//! Extended Grapheme Clusters.
//!
//! The text between extended grapheme cluster boundaries as
//! specified by [UAX #29, "Unicode Text Segmentation"][0].
//!
//! [0]: https://www.unicode.org/reports/tr29/
//

mod nonul;
#[cfg(feature = "alloc")]
mod string;
mod string_u8;
mod r#trait;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{nonul::*, r#trait::*, string_u8::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::string::*;
}
