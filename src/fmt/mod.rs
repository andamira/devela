// devela::fmt
//
//! Formatting, extends [`alloc::fmt`].
//

mod misc;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "fmt")))]
pub use misc::*;

// SAFETY: unsafe blocks are ported verbatim from the throughly tested `itoa` crate.
#[cfg(feature = "unsafe_fmt")]
mod int_buf;
#[cfg(feature = "unsafe_fmt")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "fmt_unsafe")))]
pub use int_buf::{IntBuf, IntBufAble};
