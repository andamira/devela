// devela::data::collections::array::d1
//
//! 1-dimensional array
//

/* always compiled */

// without re-exports
mod impl_traits;
mod methods;

mod definitions;
pub use definitions::*;

/* feature-gated */

#[cfg(feature = "unsafe_array")]
mod uninit;

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
