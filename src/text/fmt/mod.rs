// devela::text::fmt
//
//! Formatting, extends `std::`[`fmt`].
//!
//! [`fmt`]: std::fmt
//

/* contains always compiled items */

/* feature-gated */

#[cfg(feature = "unsafe_text")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_text")))]
mod int_buf;
mod misc;

// re-exports private sub-modules
#[cfg(feature = "unsafe_text")]
pub use int_buf::{IntBuf, IntBufAble};
#[cfg(feature = "text")]
pub use misc::*;

#[cfg(feature = "text")]
pub(crate) mod all {
    pub use super::misc::*;

    #[cfg(feature = "unsafe_text")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_text")))]
    pub use super::int_buf::{IntBuf, IntBufAble};
}
