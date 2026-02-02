// devela_base_core::geom
//
#![doc = crate::_DOC_GEOM!()] // public, root
#![doc = crate::_DOC_GEOM_MODULES!()]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_GEOM_MODULES =
    crate::_doc!(modules: crate; geom: affine, dir, metric, rel, space);
}

mod _helpers;

pub mod affine; // Structure of space under translation and linear combination.
pub mod dir; // Spatial navigation and facing semantics.
pub mod metric; // Measurement of space: distances, extents, and magnitudes.
pub mod rel; // Spatial predicates and semantic relations between geometric entities.
pub mod space; // Global organization, decomposition, and structure of space.

#[cfg(feature = "fig")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "fig")))]
pub mod fig; // Concrete geometric figures and objects.

crate::structural_mods! { // _pub_mods, _crate_internals, _workspace_internals
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            affine::_all::*,
            dir::_all::*,
            metric::_all::*,
            rel::_all::*,
            space::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_GEOM_MODULES;
    }
    _workspace_internals {
        pub use super::_helpers::*;
    }
}
