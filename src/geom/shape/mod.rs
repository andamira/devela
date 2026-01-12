// devela::geom::shape
//
//! Geometric shapes: points, angles, extents, polygons, polyhedra...
//

mod point; // Point, Points, Point2d, Point3d, VecPoints

// WIPZONE
// mod line;

crate::structural_mods! { // _mods
    _mods {
        pub use super::point::*;

        // WIPZONE
        // pub use super::line::*;
    }
}
