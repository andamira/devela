// devela::mem
//
//! Memory, extends [`core::mem`].
//!
//! It also reexports the [`bytemuck`](https://docs.rs/bytemuck) crate.
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub use always_fns::*;

/* only compiled with the `mem` feature */

/* re-exports */

// Reexported [`bytemuck`](https://docs.rs/bytemuck)'s crate.
// Gives small utilities for casting between plain data types.
#[doc(no_inline)]
#[cfg(any(feature = "bytemuck", all(feature = "mem", feature = "depend")))]
pub use crate::depend::bytemuck;

#[cfg(feature = "mem")]
mod reexports;

#[cfg(feature = "mem")]
pub use all::*;
#[cfg(feature = "mem")]
pub(crate) mod all {
    pub use super::always_fns::*;
}
