// devela::num::geom::shape
//
//! Geometric shapes: points, angles, extents, polygons, polyhedra...
//

mod point; // Point, Points, Point2d, Point3d, VecPoints

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::point::*;
        // WIPZONE
        // pub use super::line::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod line;
