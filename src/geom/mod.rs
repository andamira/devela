// devela::geom
//
#![doc = crate::_DOC_GEOM!()] // public, root
#![doc = crate::_DOC_GEOM_MODULES!()]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//! Geometry in this crate provides foundational abstractions for reasoning
//! about space, structure, and spatial relationships. It defines how
//! positions, measurements, directions, and figures are represented;
//! and how space can be organized, partitioned, and interpreted.
//!
//! The module is organized by what aspect of space is being described:
//! - affine structure (how positions combine),
//! - metric structure (how space is measured),
//! - directional structure (how orientation is defined),
//! - concrete figures (what exists in space),
//! - spatial relations (what holds between entities),
//! - global space organization (grids, layouts, topology, and motion).
//!
//! This separation enables reuse across games, simulations, layout systems, and
//! symbolic reasoning without conflating representation, measurement, and semantics.
//
// safety
#![cfg_attr(feature = "safe_geom", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_GEOM_MODULES =
    crate::_doc!(modules: crate; geom: affine, dir, metric, rel, space);
}

pub mod affine; // Structure of space under translation and linear combination.
pub mod dir; // Spatial navigation and facing semantics.
pub mod metric; // Measurement of space: distances, extents, and magnitudes.
pub mod rel; // Spatial predicates and semantic relations between geometric entities.
pub mod space; // Global organization, decomposition, and structure of space.

#[cfg(feature = "fig")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "fig")))]
pub mod fig; // Concrete geometric figures and objects.

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            affine::_all::*,
            dir::_all::*,
            metric::_all::*,
            rel::_all::*,
            space::_all::*,
        };
        #[cfg(feature = "fig")]
        pub use super::fig::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_GEOM_MODULES;
    }
}
