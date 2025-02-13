// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
//

mod metric; // Position, Distance, Extent, Stride...

#[cfg(feature = "geom")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "geom")))]
mod shape; // Point, …

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::metric::_all::*;

        #[cfg(feature = "geom")]
        pub use super::shape::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
