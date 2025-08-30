// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
#![doc = crate::_doc!(modules: crate::num; geom: linear, metric, shape)]
//

pub mod metric; // Position, Distance, Extent, Stride...

#[cfg(feature = "linear")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
pub mod linear; // Vector*, Matrix*
#[cfg(feature = "shape")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "shape")))]
pub mod shape; // Point, …

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::metric::_all::*;

        #[cfg(feature = "linear")]
        pub use super::linear::_all::*;
        #[cfg(feature = "shape")]
        pub use super::shape::_all::*;
    }
    _crate_internals {
        pub(crate) use super::metric::_crate_internals::*;
    }
}
