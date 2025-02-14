// devela::num::geom::metric
//
//! Geometric measurement and spatial relationships.
//!
//! This module defines core spatial metrics describing spatial properties
//! in an orthogonal coordinate system, enabling structured traversal
//! and measurement in geometric spaces.
//

mod angle; // Angle, AngleDirection, AngleKind
mod distance; // Distance
mod extent; // Extent, Extent2d, Extent3d
mod orientation; // Orientation
mod position; // Position
mod region; // Region
mod stride; // Stride

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{
            angle::*, distance::*, extent::*, orientation::*, position::*, region::*, stride::*,
        };
        // WIPZONE
        // pub use super::cycle::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// moc cycle;
