// devela::fig
//
//! Geometric figures and operations, spatial constructs and analysis.
//

// safety:
#![cfg_attr(feature = "safe_fig", forbid(unsafe_code))]

mod prim;
pub use prim::*;

/* feature-gated */

#[cfg(feature = "fig")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "fig")))]
mod algebra;

#[cfg(feature = "fig")]
pub use algebra::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::prim::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "fig")]
    pub use super::algebra::*;
}
