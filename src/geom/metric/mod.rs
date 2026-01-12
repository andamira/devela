// devela::geom::metric
//
#![doc = crate::_DOC_GEOM_METRIC!()]
//!
//! This module defines core spatial metrics describing spatial properties
//! in an orthogonal coordinate system, enabling structured traversal
//! and measurement in geometric spaces.
//

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::{
            Distance, Extent, Position, Region, Stride,
        };
    }
}
