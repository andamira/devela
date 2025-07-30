// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
#![doc = crate::doc_!(modules: crate::num; geom: linear, metric, shape)]
//

pub mod metric; // Position, Distance, Extent, Stride...

#[cfg(feature = "linear")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linear")))]
pub mod linear; // Vector*, Matrix*
#[cfg(feature = "shape")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "shape")))]
pub mod shape; // Point, …

crate::items! { // structural access: _pub_mods, _internals, _all
    #[allow(unused)]
    pub use _internals::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    // mod _mods {}
    mod _pub_mods { #![allow(unused)]
        pub use super::metric::_all::*;

        #[cfg(feature = "linear")]
        pub use super::linear::_all::*;
        #[cfg(feature = "shape")]
        pub use super::shape::_all::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::metric::_internals::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
