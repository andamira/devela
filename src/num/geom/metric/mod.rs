// devela::num::geom::metric
//
#![doc = crate::_DOC_NUM_GEOM_METRIC!()]
//!
//! This module defines core spatial metrics describing spatial properties
//! in an orthogonal coordinate system, enabling structured traversal
//! and measurement in geometric spaces.
//

#[cfg(feature = "metric")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
    mod angle; // Angle, AngleDirection, AngleKind
}

// WIPZONE
// mod cycle;
// mod radial_sectors;

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "metric")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
        pub use super::{
            angle::*,
        };

        // re-exports
        #[doc(inline)]
        pub use devela_base_core::{
            Distance, Extent, Orientation, Position, Region, Stride,
        };
    }
}
