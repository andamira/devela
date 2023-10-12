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
#[doc = "A wrapper to inhibit compiler from automatically calling `T`’s destructor.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::ManuallyDrop;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A wrapper type to construct uninitialized instances of `T`.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::MaybeUninit;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Opaque type representing the discriminant of an enum.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::Discriminant as EnumDiscriminant;

/* fns */

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns a value uniquely identifying the enum variant in `v`.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::discriminant as enum_discriminant;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns the ABI-required minimum alignment of the type of the value that
`val` points to in bytes.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::align_of as mem_align_of;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns the align of the pointed-to value in bytes.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::align_of_val as mem_align_of_val;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Disposes of a value.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::drop as mem_drop;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Takes ownership and “forgets” about `t` *without running its destructor*.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::forget as mem_forget;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns true if dropping values of type T matters.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::needs_drop as mem_needs_drop;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Moves `src` into `dest`, returning the previous `dest` value.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::replace as mem_replace;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns the size of a type in bytes.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::size_of as mem_size_of;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns the size of the pointed-to value in bytes.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::size_of_val as mem_size_of_val;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Swaps the values at two locations, without deinitializing either one.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::swap as mem_swap;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Replaces `dest` with `T::default()`, returning the previous `dest` value.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::take as mem_take;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Reinterprets the bits of a value of one type as another type.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::transmute as mem_transmute;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Reads `src` as having type `&Dst` without moving the contained value.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::transmute_copy as mem_transmute_copy;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Returns the value of type `T` represented by the all-zero byte-pattern.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`mem`](https://doc.rust-lang.org/core/mem)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub use core::mem::zeroed as mem_zeroed;
