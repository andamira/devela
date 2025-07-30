// devela::num::geom::metric
//
//! Geometric measurement and spatial relationships.
//!
//! This module defines core spatial metrics describing spatial properties
//! in an orthogonal coordinate system, enabling structured traversal
//! and measurement in geometric spaces.
//

mod helpers; // _impl_metric!

mod distance; // Distance
mod extent; // Extent
mod position; // Position

#[cfg(feature = "metric")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
    mod angle; // Angle, AngleDirection, AngleKind
    #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
    mod orientation; // Orientation
    #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
    mod region; // Region
    #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
    mod stride; // Stride
}

crate::items! { // structural access: _mods, _internals, _all
    #[allow(unused)]
    pub use {_internals::*, _mods::*};

    mod _mods {
        pub use super::{distance::*, extent::*, position::*};

        #[cfg(feature = "metric")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "metric")))]
        pub use super::{angle::*, orientation::*, region::*, stride::*};
        // WIPZONE
        // pub use super::cycle::*;
        // pub use super::radial_sectors::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::helpers::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod cycle;
// mod radial_sectors;
