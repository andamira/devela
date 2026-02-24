// devela::geom
//
#![doc = crate::_DOC_GEOM!()] // public, root
#![doc = crate::_DOC_GEOM_MODULES!()]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//! Foundational abstractions for reasoning about space, structure, and spatial relationships.
//
// safety
#![cfg_attr(feature = "safe_geom", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_GEOM_MODULES =
    crate::_doc!(modules: crate; geom: affine, dir, fig); // metric, rel, space
}

mod metric; // Measurement of space: distances, extents, and magnitudes.
mod rel; // Spatial predicates and semantic relations between geometric entities. WIP
mod space; // Global organization, decomposition, and structure of space. WIP

pub mod affine; // Structure of space under translation and linear combination.
pub mod dir; // Spatial navigation and facing semantics.

#[cfg(feature = "fig")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "fig")))]
pub mod fig; // Concrete geometric figures and objects.

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            metric::_all::*,
            rel::_all::*,
            space::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            affine::_all::*,
            dir::_all::*,
        };
        #[cfg(feature = "fig")]
        pub use super::fig::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_GEOM_MODULES;
    }
}
