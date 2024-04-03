// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
//

mod prim;
pub use prim::*;

#[cfg(feature = "num_geom")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_geom")))]
mod algebra;
#[cfg(feature = "num_geom")]
pub use algebra::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::prim::*;

    #[doc(inline)]
    #[cfg(feature = "num_geom")]
    #[allow(unused_imports)]
    pub use super::algebra::*;
}
