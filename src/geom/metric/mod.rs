// devela::geom::metric
//
#![doc = crate::_DOC_GEOM_METRIC!()] // public
#![doc = crate::_doc!(modules: crate::geom; metric)]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//!
//! This module defines core spatial metrics describing spatial properties
//! in an orthogonal coordinate system, enabling structured traversal
//! and measurement in geometric spaces.
//

// mod axes; // TODO
// mod cycle; // CycleOffset, Spacing // MAYBE
mod distance; // Distance[1|2|3]
mod extent; // Extent[1|2|3]
mod position; // Position[1|2|3]
mod region; // Region[1|2|3], RegionS[1|2|3]
mod stride; // Stride[1|2|3]

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // axes::*, // TODO
            // cycle::*, // MAYBE
            distance::*,
            extent::*,
            position::*,
            region::*,
            stride::*,
        };
    }
}
