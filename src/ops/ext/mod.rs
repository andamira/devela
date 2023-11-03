// devela::ops::ext
//
//!
//

#[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
#[cfg(any(feature = "std", feature = "libm"))]
mod float;

#[doc(inline)]
#[cfg(any(feature = "std", feature = "libm"))]
pub use float::*;
