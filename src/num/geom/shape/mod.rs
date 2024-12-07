// devela::num::geom::shape
//
//! Geometric shapes: points, angles, extents, polygons, polyhedra...
//

mod angle; // Angle, AngleDirection, AngleKind
mod extent; // Extent, Extent2d, Extent3d
mod point; // Point, Points, Point2d, Point3d, VecPoints

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{angle::*, extent::*, point::*};
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
