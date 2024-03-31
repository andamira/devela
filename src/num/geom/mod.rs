// devela::num::geom
//
//! Geometric figures and operations, spatial constructs and analysis.
//

mod prim;
pub use prim::*;

/* feature-gated */

#[cfg(feature = "num_geom")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_geom")))]
mod algebra;

#[cfg(feature = "num_geom")]
pub use algebra::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::prim::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "num_geom")]
    pub use super::algebra::*;
}
