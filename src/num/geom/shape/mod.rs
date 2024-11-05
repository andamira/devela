// devela::num::geom::shape
//
//! Geometric shapes: points, angles, extents, polygons, polyhedra...
//

mod angle;
mod extent;
mod point;
pub use {angle::*, extent::*, point::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused)]
    pub use super::{angle::*, extent::*, point::*};
}
