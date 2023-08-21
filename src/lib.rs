// devela::lib
//
//! Development extensions for the Rust Standard Library.
//!
//! # Features
//!
//! - `std` (default): enables functionality that depends on the standard library.
//!   Disabling it makes the crate `no_std` compatible.
//! - `alloc`: enables functionality that depends on allocation. Included in `std`.
//! - `no-std`: enables functionality incompatible with `std` (unused).
//! ---
//! - `safe`: forbids all `unsafe` code at the crate level.
//! - `unsafe`: meta feature enabling every specific unsafe feature:
//!   - `unsafe_cmp`: enables const floating-point comparison in [`cmp`],
//!      using [`transmute`] for constant access to the bits.
//!   - `unsafe_convert`: enables using [`MaybeUninit`] for array
//!     initialization in [`slice_into_array`].
//!   - `unsafe_fmt`: provides [`IntBuf`] and [`IntBufable`] in [`fmt`].
//!     Unsafe blocks are ported verbatim from [`itoa`].
//!   - `unsafe_num`: enables `new_unchecked` and implements
//!     [`bytemuck`] traits for new types defined in [`num`].
//! ---
//! - `bytemuck`: implements several [`bytemuck`] traits for `NonSpecific*`,
//!   if the `unsafe_num` feature is enabled.
//!
//! [`IntBuf`]: fmt::IntBuf
//! [`IntBufable`]: fmt::IntBufAble
//! [`slice_into_array`]: convert::collection::slice_into_array
//! [`MaybeUninit`]: core::mem::MaybeUninit
//! [`transmute`]: core::mem::transmute
//! [`itoa`]: https://docs.rs/itoa
//

#![warn(clippy::all)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");
#[cfg(all(
    feature = "safe",
    any(
        feature = "unsafe",
        feature = "unsafe_cmp",
        feature = "unsafe_convert",
        feature = "unsafe_num",
        feature = "unsafe_fmt"
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate devela_macros;

pub mod boxed;
pub mod cmp;
pub mod codegen;
pub mod convert;
pub mod fmt;
pub mod mem;
pub mod num;
pub mod ops;
pub mod option;
pub mod path;
pub mod result;
pub mod slice;
pub mod string;

/// All the types and traits are flat reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{
        boxed::*,
        cmp::*,
        codegen::*,
        convert::{collection::*, primitive::*},
        fmt::*,
        // mem::{},
        num::*,
        ops::*,
        option::*,
        path::*,
        result::*,
        slice::*,
        string::*,
    };

    #[doc(inline)]
    pub use devela_macros::compile;

    #[doc(no_inline)]
    pub use ::az::*;
    #[doc(no_inline)]
    pub use ::paste::*;

    #[doc(no_inline)]
    #[cfg(feature = "bytemuck")]
    pub use ::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};
}

/// The common prelude.
pub mod prelude {
    pub use crate::{
        convert::{FromPrimitives, IntoPrimitives},
        num::*,
        ops::{Also, Apply},
        option::OptionExt,
        result::ResultExt,
        slice::{SliceExt, SliceExtMut},
    };

    #[doc(no_inline)]
    pub use core::num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    };
}
