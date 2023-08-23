// devela::mem
//
//! Memory, extends [`core::mem`].
//

/// Reexported [`bytemuck`](https://docs.rs/bytemuck) crate.
/// Gives small utilities for casting between plain data types.
///
#[doc(inline)]
#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "bytemuck")))]
pub use ::bytemuck;
#[cfg(feature = "bytemuck")]
#[doc(no_inline)]
pub use ::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};
