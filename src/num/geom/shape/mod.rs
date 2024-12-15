// devela::num::geom::shape
//
//! Geometric shapes: points, angles, extents, polygons, polyhedra...
//

mod angle; // Angle, AngleDirection, AngleKind
mod extent; // Extent, Extent2d, Extent3d
mod point; // Point, Points, Point2d, Point3d, VecPoints

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{angle::*, extent::*, point::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
