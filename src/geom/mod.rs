// devela::geom
//
#![doc = crate::_DOC_GEOM!()]
#![doc = crate::_DOC_GEOM_MODULES!()]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(feature = "safe_geom", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_GEOM_MODULES =
    crate::_doc!(modules: crate; geom: linear, metric, shape); // field
}

pub mod dir; // Angle, AngleKind, Orientation...
pub mod metric; // Position, Distance, Extent, Stride...

#[cfg(feature = "linear")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
pub mod linear; // Vector*, Matrix*
#[cfg(feature = "shape")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "shape")))]
pub mod shape; // Point, …

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            dir::_all::*,
            metric::_all::*,
        };

        #[cfg(feature = "linear")]
        pub use super::linear::_all::*;
        #[cfg(feature = "shape")]
        pub use super::shape::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_GEOM_MODULES;
    }
}
