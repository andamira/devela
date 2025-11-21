// devela::num::geom
//
#![doc = crate::_DOC_NUM_GEOM!()]
#![doc = crate::_doc!(modules: crate::num; geom: linear, metric, shape)]
//

pub mod dir; // Angle, AngleKind, Orientation...
pub mod metric; // Position, Distance, Extent, Stride...

#[cfg(feature = "linear")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
pub mod linear; // Vector*, Matrix*
#[cfg(feature = "shape")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "shape")))]
pub mod shape; // Point, …

crate::structural_mods! { // _pub_mods
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
}
