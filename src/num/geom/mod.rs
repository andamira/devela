// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
//

mod shape;
pub use shape::*;
//
pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::shape::*;
}
