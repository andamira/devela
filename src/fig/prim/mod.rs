// devela::fig::prim
//
//! Geometric primitives: points, lines, angles, polygons, polyhedra...
//

/* always compiled */

mod angle;

#[allow(unused)]
pub use angle::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused)]
    pub use super::angle::*;
}
