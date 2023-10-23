// devela::text::fmt
//
//! Formatting, extends `std::` [`fmt`][std::fmt].
//

/* always compiled for internal use */

/* only compiled with the `char` feature */

mod misc;

#[cfg(feature = "unsafe_text")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_text")))]
mod int_buf;

/* re-exports */

#[cfg(feature = "text")]
pub use all::*;
#[cfg(feature = "text")]
pub(crate) mod all {
    pub use super::misc::*;

    #[cfg(feature = "unsafe_text")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_text")))]
    pub use super::int_buf::{IntBuf, IntBufAble};
}
