// devela::lib
//
//! Development extensions for the Rust Standard Library.
//!
//! ## Crate features
//!
//! - `std` (default): enables functionality that depends on the standard library.
//!   Disabling it makes the crate `no_std` compatible.
//! - `alloc`: enables functionality that depends on allocation. Included in `std`.
//! - `no-std`: enables functionality incompatible with `std` (unused).
//! ---
//! - `safe`: forbids `unsafe` code at the crate level.
//!   - `unsafe`: meta feature that enables all the specific unsafe features:
//!   - `unsafe_int_buf`: enables the [`IntBuf`] struct and the [`IntBufable`] trait.
//!   - `unsafe_non_specific`: enables unsafe in the `NonSpecific*` types.
//!   - `unsafe_uninit`: enables using [`MaybeUninit`][core::mem::MaybeUninit].
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

/// Reexported from the [`paste`](https://docs.rs/paste) crate.
/// Allows to paste identifiers in a macro.
pub use paste::paste;

pub mod boxed;
pub mod cmp;
pub mod convert;
pub mod fmt;
pub mod num;
pub mod ops;
pub mod option;
pub mod path;
pub mod result;
pub mod slice;
pub mod string;

/// All the types and traits in the same place.
///
/// Everything is re-exported from here.
pub mod all {
    #[doc(inline)]
    pub use super::{
        boxed::*,
        cmp::*,
        convert::{az::*, collection::*, primitive::*},
        fmt::*,
        num::*,
        ops::*,
        option::*,
        path::*,
        result::*,
        slice::*,
        string::*,
    };

    /// Reexported from the [`paste`](https://docs.rs/paste) crate.
    /// Allows to paste identifiers in a macro.
    pub use super::paste;
}

/// The common prelude.
pub mod prelude {
    pub use super::{
        convert::{FromPrimitives, IntoPrimitives},
        ops::{Also, Apply},
        option::OptionExt,
        result::ResultExt,
        slice::SliceExt,
    };
}
