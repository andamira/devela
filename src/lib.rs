// devela::lib
//
//! Development extensions for the Rust Standard Library.
//

// warnings
#![warn(clippy::all)]
#![allow(clippy::wrong_self_convention)] // allow `is_` methods with owned self
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
        feature = "unsafe_ascii",
        feature = "unsafe_char",
        feature = "unsafe_cmp",
        feature = "unsafe_convert",
        feature = "unsafe_fmt",
        feature = "unsafe_mem",
        feature = "unsafe_num",
        feature = "unsafe_os",
        feature = "unsafe_linux",
        feature = "unsafe_str",
        feature = "unsafe_string",
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

extern crate devela_macros;

/* sub-modules */

#[cfg(feature = "ascii")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ascii")))]
pub mod ascii;
#[cfg(not(feature = "ascii"))]
pub(crate) mod ascii; // the "ascii" feature is disabled

#[cfg(feature = "char")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "char")))]
pub mod char;
// #[cfg(not(feature = "char"))]
// pub(crate) mod char; // the "char" feature is disabled

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
pub mod str;
pub mod string;
pub mod sync;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod thread;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    #[cfg(feature = "ascii")]
    pub use super::ascii::*;

    #[doc(inline)]
    #[cfg(feature = "char")]
    pub use super::char::*;

    #[doc(inline)]
    pub use super::{
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
        str::*,
        string::all::*,
        sync::all::*,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::thread::*;

    #[doc(inline)]
    pub use devela_macros::{cif, compile, compile_attr};

    pub use ::az;
    pub use ::bytemuck;
}

/// The common prelude.
pub mod prelude {
    pub use crate::{
        convert::{FromPrimitives, IntoPrimitives},
        ops::{Also, Apply},
        option::OptionExt,
        result::ResultExt,
        slice::{SliceExt, SliceExtMut},
        str::StrExt,
    };

    #[cfg(feature = "alloc")]
    pub use crate::string::StringExt;
}

/// Documentation on features.
pub mod _doc {
    #![doc = include_str!("./Doc.md")]
}
