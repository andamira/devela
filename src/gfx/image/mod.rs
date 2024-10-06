// devela::gfx::image
//
//! Image manipulation.
// #![doc = crate::code::doc_!(modules: crate::gfx; image: pnm)]
// #![doc = crate::code::doc_!(newline)]
//

mod pnm;
pub use pnm::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::pnm::*;
}