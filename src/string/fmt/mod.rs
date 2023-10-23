// devela::string::fmt
//
//! Formatting, extends `std::` [`fmt`][std::fmt].
//

/* always compiled for internal use */

/* only compiled with the `char` feature */

mod misc;

#[cfg(feature = "unsafe_string")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_string")))]
mod int_buf;

/* re-exports */

#[cfg(feature = "string")]
pub use all::*;
#[cfg(feature = "string")]
pub(crate) mod all {
    pub use super::misc::*;

    #[cfg(feature = "unsafe_string")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_string")))]
    pub use super::int_buf::{IntBuf, IntBufAble};
}
