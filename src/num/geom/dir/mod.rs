// devela::num::geom::dir
//
#![doc = crate::_DOC_NUM_GEOM_DIR!()]
//!
//! This module defines the core primitives that describe how objects
//! point, turn, and transform within a geometric space. It covers both
//! continuous orientations and their discrete, symmetry-based
//! classifications.
//!
//! The types here provide the directional layer of geometry:
//! normalized vectors, angular notions, canonical sectors,
//! axis-aligned symmetries, and reversible orientation transforms.
//!
//! These constructs form the basis for navigation, rotation,
//! alignment, spatial reasoning, and any system where direction
//! drives behavior.
//

mod angle; // Angle, AngleDirection, AngleKind

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            angle::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::{
            Orientation,
        };
    }
}
