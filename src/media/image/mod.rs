// devela::media::image
//
//! Image manipulation.
// #![doc = crate::doc_!(modules: crate::media; image: pnm)]
// #![doc = crate::doc_!(newline)]
//
// safety
#![cfg_attr(feature = "safe_image", forbid(unsafe_code))]

mod error;
mod pnm;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{error::*, pnm::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
