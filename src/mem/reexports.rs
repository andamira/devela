// devela::mem
//
//! Reexported items from `core`.
//

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Types with a constant size known at compile time.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`marker`](https://doc.rust-lang.org/core/marker)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::marker::Sized;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns the size of a type in bytes.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::size_of;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns the size of the pointed-to value in bytes.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::size_of_val;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Swaps the values at two mutable locations, without deinitializing either one."]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::swap;
