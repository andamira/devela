// devela/src/geom/dir/mod.rs
//
#![doc = crate::_DOC_GEOM_DIR!()] // public
#![doc = crate::_doc!(modules: crate::geom; dir)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//!
//! This module distinguishes angular, component-based,
//! and region-relative forms of direction.
//!
//! ```text
//! Angle
//! ├── interpreted by ──> AngleDirection
//! └── classified by ──> AngleKind
//!
//! Orientation<T, D>     component-based spatial direction
//!
//! Boundary1d
//! Boundary2d            discrete side or face of a bounded region
//! Boundary3d
//! ```
//!
//! [`Angle`] represents a fraction of a full rotation without committing its
//! stored value to degrees or radians. [`AngleDirection`] describes rotational
//! sign, while [`AngleKind`] classifies the normalized turn as full, acute,
//! right, obtuse, straight, or reflex.
//!
//! [`Orientation`] represents a direction through dimensional components,
//! without an absolute position or inherent magnitude. Orientations are
//! commonly normalized, but the type does not enforce normalization.
//!
//! [`Boundary1d`], [`Boundary2d`], and [`Boundary3d`] identify sides or faces
//! relative to a bounded region. They are discrete labels and do not imply
//! movement, distance, or a free-standing directional vector.
//!
//! One-, two-, and three-dimensional orientation aliases are provided together
//! with the [`ori!`] construction macro.
//!
//! # Boundaries
//!
//! - Positions, distances, extents, regions, and strides belong to [`geom::metric`].
//! - Coordinate frames and transformations belong to [`geom::affine`].
//! - Concrete geometric objects belong to [`geom::fig`].
//!
//! They define no navigation policy, object-facing state,
//! global coordinate frame, zero-angle axis, or transform pipeline.
//!
//! [`geom::fig`]: crate::geom::fig
//! [`geom::affine`]: crate::geom::affine
//! [`geom::metric`]: crate::geom::metric

//

mod angle; // Angle, AngleDirection, AngleKind
mod boundary; // Boundary[1|2|3]d
mod macros; // ori!
// mod nav; // Spatial navigation and facing semantics. WIP
mod orientation; // Orientation[1|2|3]
// mod rot_sector; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            angle::*,
            boundary::*,
            macros::ori,
            // nav::_all::*,
            orientation::*,
            // rot_sector::*,
        };
    }
}
