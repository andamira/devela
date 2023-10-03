// devela::lib
//
//! Development extensions for the Rust Standard Library.
//

// warnings
#![warn(missing_docs, clippy::all)]
#![allow(clippy::wrong_self_convention)] // allow `is_` methods with owned self
// environment, safety, nightly
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
        feature = "unsafe", // includes all below
        feature = "unsafe_ascii",
        feature = "unsafe_char",
        feature = "unsafe_cmp",
        feature = "unsafe_convert",
        feature = "unsafe_fmt",
        feature = "unsafe_mem",
        feature = "unsafe_num",
        feature = "unsafe_ops",
        feature = "unsafe_os", // includes: unsafe_{linux}
        feature = "unsafe_linux", //
        feature = "unsafe_str",
        feature = "unsafe_string",
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

extern crate devela_macros;

/* root modules */

#[cfg(any(feature = "ascii", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ascii")))]
pub mod ascii;
#[cfg(all(not(feature = "ascii"), not(test)))]
pub(crate) mod ascii; // the "ascii" feature is disabled

#[cfg(any(feature = "char", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "char")))]
pub mod char;
#[cfg(all(not(feature = "char"), not(test)))]
pub(crate) mod char; // the "char" feature is disabled

#[cfg(any(feature = "cmp", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "cmp")))]
pub mod cmp;
#[cfg(all(not(feature = "cmp"), not(test)))]
pub(crate) mod cmp; // the "cmp" feature is disabled

pub mod codegen;

#[cfg(any(feature = "convert", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "convert")))]
pub mod convert;
#[cfg(all(not(feature = "convert"), not(test)))]
pub(crate) mod convert; // the "convert" feature is disabled

#[cfg(any(feature = "fmt", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "fmt")))]
pub mod fmt;
// #[cfg(all(not(feature = "fmt"), not(test)))]
// pub(crate) mod fmt; // the "fmt" feature is disabled

#[cfg(any(feature = "mem", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub mod mem;
#[cfg(all(not(feature = "mem"), not(test)))]
pub(crate) mod mem; // the "mem" feature is disabled

#[cfg(any(feature = "num", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
pub mod num;
#[cfg(all(not(feature = "num"), not(test)))]
pub(crate) mod num; // the "num" feature is disabled

#[cfg(any(feature = "ops", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ops")))]
pub mod ops;
#[cfg(all(not(feature = "ops"), not(test)))]
pub(crate) mod ops; // the "ops" feature is disabled

pub mod option;
pub mod os;
pub mod path;
pub mod result;

#[cfg(any(feature = "slice", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "slice")))]
pub mod slice;
#[cfg(all(not(feature = "slice"), not(test)))]
pub(crate) mod slice; // the "slice" feature is disabled

#[cfg(any(feature = "str", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "str")))]
pub mod str;
#[cfg(all(not(feature = "str"), not(test)))]
pub(crate) mod str; // the "str" feature is disabled

#[cfg(any(feature = "string", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "string")))]
pub mod string;
#[cfg(all(not(feature = "string"), not(test)))]
pub(crate) mod string; // the "string" feature is disabled

#[cfg(any(feature = "sync", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub mod sync;
#[cfg(all(not(feature = "sync"), not(test)))]
pub(crate) mod sync; // the "sync" feature is disabled

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod thread;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    #[cfg(feature = "ascii")]
    pub use super::ascii::all::*;

    #[doc(inline)]
    #[cfg(feature = "char")]
    pub use super::char::all::*;

    #[doc(inline)]
    #[cfg(feature = "cmp")]
    pub use super::cmp::all::*;

    #[doc(inline)]
    #[cfg(feature = "convert")]
    pub use super::convert::all::*;
    #[cfg(feature = "az")]
    pub use ::az;

    #[doc(inline)]
    #[cfg(feature = "fmt")]
    pub use super::fmt::all::*;

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::mem::all::*;
    #[cfg(feature = "bytemuck")]
    pub use ::bytemuck;

    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::num::all::*;

    #[doc(inline)]
    #[cfg(feature = "ops")]
    pub use super::ops::all::*;

    #[doc(inline)]
    #[cfg(feature = "slice")]
    pub use super::slice::all::*;

    #[doc(inline)]
    #[cfg(feature = "str")]
    pub use super::str::all::*;

    #[doc(inline)]
    #[cfg(feature = "string")]
    pub use super::string::all::*;

    #[doc(inline)]
    #[cfg(feature = "sync")]
    pub use super::sync::all::*;

    #[doc(inline)]
    pub use super::{codegen::all::*, option::*, os::all::*, path::*, result::*};

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::thread::*;

    #[doc(inline)]
    pub use devela_macros::{cif, compile, compile_attr};
}

/// The common prelude.
pub mod prelude {
    #[cfg(feature = "convert")]
    pub use crate::convert::all::{FromPrimitives, IntoPrimitives};

    #[cfg(feature = "ops")]
    pub use crate::ops::all::{Also, Apply};

    #[cfg(feature = "slice")]
    pub use crate::slice::all::{SliceExt, SliceExtMut};

    #[cfg(feature = "str")]
    pub use crate::str::all::StrExt;

    pub use crate::{option::OptionExt, result::ResultExt};

    #[cfg(all(feature = "string", feature = "alloc"))]
    pub use crate::string::StringExt;
}

/// Documentation on features.
pub mod _doc {
    #![doc = include_str!("./Doc.md")]
}
