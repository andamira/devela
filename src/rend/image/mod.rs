// devela::rend::image
//
//! Image manipulation.
// #![doc = crate::code::doc_!(modules: crate::rend; image: pnm)]
// #![doc = crate::code::doc_!(newline)]
//

mod error;
mod pnm;
pub use {error::*, pnm::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{error::*, pnm::*};
}
