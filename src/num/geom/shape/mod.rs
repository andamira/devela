// devela::num::geom::shape
//
//! Geometric shapes: points, angles, extents, polygons, polyhedra...
//

mod angle;
pub use angle::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused)]
    pub use super::angle::*;
}
