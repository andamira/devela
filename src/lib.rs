// devela::lib
//
//! Development extensions for the Rust Standard Library.
//!
// #![doc = include_str!("./Lib.md")]
//

// warnings
#![warn(clippy::all)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
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
// deprecated
deprecate_feature![old: "no-std", new: "no_std", since: "0.8.0"];

extern crate devela_macros;

pub mod ascii;
pub mod cmp;
pub mod codegen;
pub mod convert;
pub mod fmt;
pub mod mem;
pub mod num;
pub mod ops;
pub mod option;
pub mod os;
pub mod path;
pub mod result;
pub mod slice;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod thread;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{
        ascii::*,
        cmp::*,
        codegen::all::*,
        convert::{collection::*, primitive::*},
        fmt::*,
        mem::all::*,
        num::*,
        ops::*,
        option::*,
        os::all::*,
        path::*,
        result::*,
        slice::*,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::thread::*;

    #[doc(inline)]
    pub use devela_macros::{cif, compile, compile_attr};

    pub use ::az;
    pub use ::bytemuck;

    #[doc(no_inline)]
    pub use core::num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    };
}

/// The common prelude.
pub mod prelude {
    pub use crate::{
        convert::{FromPrimitives, IntoPrimitives},
        ops::{Also, Apply},
        option::OptionExt,
        result::ResultExt,
        slice::{SliceExt, SliceExtMut},
    };
}

/// Documentation on features.
pub mod _features {
    #![doc = include_str!("./Doc.md")]
}
