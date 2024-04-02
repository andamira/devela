// devela::data::collections::array::d1
//
//! 1-dimensional array
//

/* always compiled */

// without re-exports
mod impl_traits;
mod methods;
// with re-exports
mod definitions;
#[allow(unused_imports)]
pub use definitions::*;

/* feature-gated */

#[cfg(feature = "unsafe_array")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_array")))]
mod uninit;
#[allow(unused_imports)]
#[cfg(feature = "unsafe_array")]
pub use uninit::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::definitions::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "unsafe_array")]
    pub use super::uninit::*;
}
