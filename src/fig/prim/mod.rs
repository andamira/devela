// devela::fig::prim
//
//! Geometric primitives: points, lines, angles, polygons, polyhedra...
//

/* always compiled */

mod angle;
mod extent;

#[allow(unused)]
pub use {angle::*, extent::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused)]
    pub use super::{angle::*, extent::*};
}
