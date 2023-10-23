// devela::string::egc
//
//! Extended Grapheme Clusters.
//!
//! The text between extended grapheme cluster boundaries as
//! specified by [UAX #29, "Unicode Text Segmentation"][0].
//!
//! [0]: https://www.unicode.org/reports/tr29/
//

mod array_string;
mod non_nul;
#[cfg(feature = "alloc")]
mod string;
mod r#trait;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array_string::*, non_nul::*, r#trait::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::string::*;
}
