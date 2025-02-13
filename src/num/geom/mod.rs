// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
#![doc = crate::doc_!(modules: crate::num; geom: metric, shape)]
//

pub mod metric; // Position, Distance, Extent, Stride...

#[cfg(feature = "geom")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "geom")))]
pub mod shape; // Point, …

crate::items! { // structural access: _mods, _all
    // #[allow(unused)]
    // pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    // mod _mods {}
    mod _pub_mods { #![allow(unused)]
        pub use super::metric::_all::*;

        #[cfg(feature = "geom")]
        pub use super::shape::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
