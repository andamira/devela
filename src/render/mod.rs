// devela::render
//
//! Graphics rendering and processing.
//

/* contains always compiled items */

#[cfg(feature = "render")]
pub mod color;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "render")]
pub use color::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "render")]
    pub use super::color::*;
}
