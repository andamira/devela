// devela::data::collections::array::d1
//
//! 1-dimensional array
//

// without re-exports
mod impl_traits;
mod methods;
// with re-exports
mod definitions;
#[allow(unused_imports)]
pub use definitions::*;

#[cfg(feature = "unsafe_array")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_array")))]
mod uninit;
#[allow(unused_imports)]
#[cfg(feature = "unsafe_array")]
pub use uninit::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::definitions::*;

    #[doc(inline)]
    #[cfg(feature = "unsafe_array")]
    pub use super::uninit::*;
}
