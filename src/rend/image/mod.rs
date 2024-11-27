// devela::rend::image
//
//! Image manipulation.
// #![doc = crate::doc_!(modules: crate::rend; image: pnm)]
// #![doc = crate::doc_!(newline)]
//

// safety:
#![cfg_attr(feature = "safe_image", forbid(unsafe_code))]

mod error;
mod pnm;
pub use {error::*, pnm::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{error::*, pnm::*};
}
