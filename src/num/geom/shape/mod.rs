// devela::num::geom::shape
//
//! Geometric shapes: points, angles, extents, polygons, polyhedra...
//

mod angle;
mod extent;
pub use {angle::*, extent::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused)]
    pub use super::{angle::*, extent::*};
}
