// devela/src/geom/mod.rs
//
#![doc = crate::_DOC_GEOM!()] // public, root
#![doc = crate::_DOC_GEOM_MODULES!()]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//! Foundational abstractions for reasoning about space, structure, and spatial relationships.
//!
//! ## Coordinate roles
//!
//! Affine geometry models the algebra of locations and displacements:
//! - [`Point`] is a location on which vectors act.
//! - [`Vector`] is a directed displacement or linear-space element.
//!
//! Metric geometry provides domain-oriented spatial quantities:
//! - [`Position`] is an absolute coordinate location.
//! - [`Distance`] is a non-oriented component-wise separation.
//! - [`Extent`] is an origin-independent component-wise size.
//!
//! `Point` and `Position` may share a representation while serving different APIs.
//! Semantic conversion between them is explicit and lossless.
//!
//! [`Vector`]: crate::Vector
//
// safety
#![cfg_attr(feature = "safe_geom", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_GEOM_MODULES =
    crate::_doc!(modules: crate; geom: affine, dir, fig, metric, space); // rel
}

mod _helper; // _geom_dim_impl_common!, _geom_dim_cast_ctor!

pub mod affine; // Structure of space under translation and linear combination.
pub mod dir; // Spatial navigation and facing semantics.
pub mod metric; // Measurement of space: distances, extents, and magnitudes.
mod rel; // Spatial predicates and semantic relations between geometric entities. WIP
pub mod space; // Global organization, decomposition, and structure of space.

#[cfg(feature = "fig")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "fig")))]
pub mod fig; // Concrete geometric figures and objects.

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            rel::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            affine::_all::*,
            dir::_all::*,
            metric::_all::*,
            space::_all::*,
        };
        #[cfg(feature = "fig")]
        pub use super::fig::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_GEOM_MODULES;
        pub(crate) use super::_helper::{
            _geom_dim_impl_common, _geom_dim_define_macro,
        };
    }
    _hidden {
        pub use super::_helper::_geom_dim_cast_ctor;
    }
}
