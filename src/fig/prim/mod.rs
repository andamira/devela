// devela::fig::prim
//
//! Geometric primitives: points, lines, angles, polygons, polyhedra...
//

/* always compiled */

mod extent;

mod angle;

pub use {angle::*, extent::*};

/* feature-gated */

#[cfg(feature = "fig")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "fig")))]
mod point;

#[cfg(feature = "fig")]
#[allow(unused)]
pub use point::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused)]
    pub use super::{angle::*, extent::*};

    // feature-gated
    #[cfg(feature = "fig")]
    #[doc(inline)]
    pub use super::point::*;
}
