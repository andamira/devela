// devela
//
//! A varied collection of [`Rust`] development helper & extension utilities.
//!
//! [`Rust`]: https://www.rust-lang.org/
//

#![warn(clippy::all)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate devela_macros;
pub use devela_macros::compile;

/// Reexported [`az`](https://docs.rs/az) crate. Provides casts and checked casts.
#[doc(inline)]
pub use az;
/// Reexported from the [`paste`](https://docs.rs/paste) crate.
/// Allows to paste identifiers in a macro.
pub use paste::paste;

mod apply;
mod convert;
mod ext;
mod format;
mod non_specific;
mod ops;
mod primitive;
mod project;
mod slice;
mod string;
mod sugar;

pub use {
    apply::{Also, Apply},
    convert::slice_into_array,
    ext::{OptionExt, ResultExt},
    format::format_buf,
    non_specific::{
        NonMaxI128, NonMaxI16, NonMaxI32, NonMaxI64, NonMaxI8, NonMaxIsize, NonMaxU128, NonMaxU16,
        NonMaxU32, NonMaxU64, NonMaxU8, NonMaxUsize, NonMinI128, NonMinI16, NonMinI32, NonMinI64,
        NonMinI8, NonMinIsize, NonMinU128, NonMinU16, NonMinU32, NonMinU64, NonMinU8, NonMinUsize,
        NonSpecificI128, NonSpecificI16, NonSpecificI32, NonSpecificI64, NonSpecificI8,
        NonSpecificIsize, NonSpecificU128, NonSpecificU16, NonSpecificU32, NonSpecificU64,
        NonSpecificU8, NonSpecificUsize,
    },
    ops::{pclamp, pmax, pmin},
    primitive::*,
    slice::{subslice_left, subslice_middle, subslice_right},
};

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use {
    convert::{slice_into_vec, try_slice_into_vec, try_vec_into_vec, vec_into_vec},
    format::AltDebug,
    string::{counter_string, indent},
    sugar::bx,
};

#[cfg(feature = "std")]
pub use project::{crate_root, crate_root_string};
