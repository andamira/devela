// devela::text::fmt
//
//! Formatting, extends `std::`[`fmt`].
//!
//! [`fmt`]: std::fmt
//

/* modules */

// always compiled, private
mod misc;
mod num_to_str;

// feature-gated, private
// #[cfg(feature = "unsafe_text")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_text")))]
// mod int_buf;

/* re-exports */

// always compiled, private
pub use {misc::*, num_to_str::*};

// feature-gated, private
// #[cfg(feature = "unsafe_text")]
// pub use int_buf::{IntBuf, IntBufAble};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{misc::*, num_to_str::*};

    // feature-gated
    // #[doc(inline)]
    // #[cfg(feature = "unsafe_text")]
    // #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_text")))]
    // pub use super::int_buf::{IntBuf, IntBufAble};
}
