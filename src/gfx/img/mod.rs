// devela::gfx::img
//
//! Image formats and manipulation.
//

/* feature-gated, non-public modules */

#[cfg(feature = "gfx")]
mod pnm;

#[cfg(feature = "gfx")]
#[allow(unused_imports)]
pub use pnm::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "gfx")]
    #[allow(unused_imports)]
    pub use super::pnm::*;
}
