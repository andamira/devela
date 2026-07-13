// devela/src/geom/metric/mod.rs
//
#![doc = crate::_DOC_GEOM_METRIC!()] // public
#![doc = crate::_doc!(modules: crate::geom; metric)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//!
//! This module gives distinct spatial roles to orthogonal component values.
//!
//! ```text
//! Position    absolute location
//! Distance    relative separation
//! Extent      origin-independent size
//! Stride      per-axis traversal step
//!
//! Position + Extent ──> Region
//! Region + Stride   ──> RegionStrided
//! ```
//!
//! Although these types share dimensional component storage, their meanings differ.
//! - A [`Position`] locates something,
//! - a [`Distance`] separates locations,
//! - an [`Extent`] gives dimensional size,
//! - a [`Stride`] describes repeated stepping.
//!
//! [`Region`] combines a position and extent. [`RegionStrided`] adds traversal
//! spacing without changing the region's spatial bounds.
//!
//! The core families support any dimensionality through a const generic.
//! One-, two-, and three-dimensional aliases are provided, together with the
//! [`pos!`], [`dis!`], [`ext!`], and [`region!`] construction macros.
//!
//! # Boundaries
//!
//! - Direction, rotation, and region-relative sides belong to [`geom::dir`][crate::geom::dir].
//! - Affine points and transformations belong to [`geom::affine`][crate::geom::affine].
//! - Concrete geometric objects belong to [`geom::fig`].
//! - Abstract intervals, ratios, and other quantitative relations belong to [`num::quant`].
//! - Physical units and measured quantities belong to [`phys`][crate::phys].
//!
//! [`RegionStrided`] records spatial stepping.
//! It does not define storage, indexing, iteration order, or memory layout.
//!
//! [`geom::fig`]: crate::geom::fig
//! [`num::quant`]: crate::num::quant
//

// mod axes; // TODO
// mod cycle; // CycleOffset, Spacing // MAYBE
mod distance; // Distance[1|2|3]
mod extent; // Extent[1|2|3]
mod macros; // dis!, ext!, pos!, region!
mod position; // Position[1|2|3]
mod region; // Region[1|2|3], RegionS[1|2|3]
mod stride; // Stride[1|2|3]

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // axes::*,
            // cycle::*,
            distance::*,
            extent::*,
            macros::{dis, ext, pos, region},
            position::*,
            region::*,
            stride::*,
        };
    }
}
