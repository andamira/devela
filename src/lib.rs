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

mod apply;
mod ops;
mod project;
mod slice;
mod string;
mod sugar;

pub use {
    apply::{Also, Apply},
    ops::{pclamp, pmax, pmin},
    slice::{subslice_left, subslice_middle, subslice_right},
};

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use {string::counter_string, sugar::bx};

#[cfg(feature = "std")]
pub use project::{crate_root, crate_root_string};
