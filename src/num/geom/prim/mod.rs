// devela::num::geom::prim
//
//! Geometric primitives: points, lines, angles, polygons, polyhedra...
//

/* always compiled */

mod angle;
mod extent;
mod point;

pub use {angle::*, extent::*, point::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused)]
    pub use super::{angle::*, extent::*, point::*};
}
