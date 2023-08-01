// devela::lib
//
//! A varied collection of mostly low-level helpers & extensions for [Rust].
//!
//! [Rust]: https://www.rust-lang.org/
//!
//! ## Crate features
//!
//! - `std` (default): enables functionality that depends on the standard library.
//!   Disabling it makes the crate `no_std` compatible.
//! - `alloc`: enables functionality that depends on allocation. Included in `std`.
//!
//! - `safe` (default): forbids `unsafe` code at the crate level.
//!   Functionality that depends on unsafe code wont be available.
//! - `unsafe`: meta feature that enables all `unsafe_*` features.
//! - `unsafe_uninit`: enables using [`MaybeUninit`][core::mem::MaybeUninit].
//! - `unsafe_int_buf`: enable [`IntBuf`] struct and [`IntBufable`] trait.
//! - `unsafe_non_specific`: enables unsafe in `NonSpecific*` impls.
//!
//! [`IntBuf`]: fmt::IntBuf
//! [`IntBufable`]: fmt::IntBufAble
//

#![warn(clippy::all)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");
#[cfg(all(
    feature = "safe",
    any(
        feature = "unsafe",
        feature = "unsafe_uninit",
        feature = "unsafe_int_buf",
        feature = "unsafe_non_specific"
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

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

pub mod apply;
pub mod convert;
pub mod ext;
pub mod fmt;
pub mod non_specific;
pub mod ops;
pub mod project;
pub mod slice;
pub mod string;
pub mod sugar;

/// All types and traits.
///
/// Everything is re-exported from here.
pub mod all {
    #[doc(inline)]
    pub use super::{
        apply::{Also, Apply},
        convert::{collection::*, primitive::*},
        ext::{OptionExt, ResultExt},
        fmt::format_buf,
        non_specific::{
            NonMaxI128, NonMaxI16, NonMaxI32, NonMaxI64, NonMaxI8, NonMaxIsize, NonMaxU128,
            NonMaxU16, NonMaxU32, NonMaxU64, NonMaxU8, NonMaxUsize, NonMinI128, NonMinI16,
            NonMinI32, NonMinI64, NonMinI8, NonMinIsize, NonMinU128, NonMinU16, NonMinU32,
            NonMinU64, NonMinU8, NonMinUsize, NonSpecificI128, NonSpecificI16, NonSpecificI32,
            NonSpecificI64, NonSpecificI8, NonSpecificIsize, NonSpecificU128, NonSpecificU16,
            NonSpecificU32, NonSpecificU64, NonSpecificU8, NonSpecificUsize,
        },
        ops::{pclamp, pmax, pmin},
        slice::{subslice_left, subslice_middle, subslice_right},
    };

    // macros
    /// Reexported from the [`paste`](https://docs.rs/paste) crate.
    /// Allows to paste identifiers in a macro.
    pub use super::paste;
    #[doc(inline)]
    pub use super::{
        fmt::iformat, // format_buf
        project::manifest_dir,
        sugar::{iif, rfs},
    };

    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use super::{
        fmt::AltDebug,
        string::{counter_string, indent},
        sugar::{bx, S},
    };

    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::{
        project::{crate_root, crate_root_string},
        sugar::cdbg,
    };
}
