// devela
//
//! Rust development helper & extension utilities.
//

#![warn(clippy::all)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");

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
mod format;
mod ops;
mod project;
mod slice;
mod string;
mod sugar;

pub use {
    apply::{Also, Apply},
    format::format_buf,
    ops::{pclamp, pmax, pmin},
    slice::{subslice_left, subslice_middle, subslice_right},
};

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use {string::counter_string, sugar::bx};

#[cfg(feature = "std")]
pub use project::{crate_root, crate_root_string};
