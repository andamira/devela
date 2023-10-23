// devela::mem
//
//! Memory, extends `std::`[`mem`][std::mem].
//

/* always compiled for internal use */

mod always_fns;
mod storage;

#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub use {always_fns::*, storage::*};

/* only compiled with the `mem` feature */

#[cfg(feature = "mem")]
mod size;

/* re-exports */

// Reexported [`bytemuck`](https://docs.rs/bytemuck)'s crate.
// Gives small utilities for casting between plain data types.
#[doc(no_inline)]
#[cfg(any(feature = "bytemuck", all(feature = "mem", feature = "depend")))]
pub use ::depend::bytemuck;

#[cfg(feature = "mem")]
mod reexports;

#[cfg(feature = "mem")]
mod r#trait;

#[cfg(feature = "mem")]
pub use all::*;
#[cfg(feature = "mem")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always_fns::*, r#trait::*, size::*, storage::*};

    #[doc(inline)]
    pub use super::reexports::*;
}
