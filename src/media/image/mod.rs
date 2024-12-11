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

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{error::*, pnm::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
    }
}
