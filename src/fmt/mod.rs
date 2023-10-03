// devela::fmt
//
//! Formatting, extends [`alloc::fmt`].
//

/* always compiled for internal use */

/* only compiled with the `char` feature */

mod misc;

#[cfg(feature = "unsafe_fmt")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_fmt")))]
mod int_buf;

/* re-exports */

#[cfg(feature = "fmt")]
pub use all::*;
#[cfg(feature = "fmt")]
pub(crate) mod all {
    pub use super::misc::*;

    #[cfg(feature = "unsafe_fmt")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_fmt")))]
    pub use super::int_buf::{IntBuf, IntBufAble};
}
