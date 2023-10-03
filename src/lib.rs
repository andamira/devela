// devela::lib
//
//! Development extensions for the Rust Standard Library.
//

// warnings
#![warn(missing_docs, clippy::all)]
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
        feature = "unsafe", // includes all below
        feature = "unsafe_ascii",
        feature = "unsafe_char",
        feature = "unsafe_cmp",
        feature = "unsafe_convert",
        feature = "unsafe_fmt",
        feature = "unsafe_mem",
        feature = "unsafe_num",
        feature = "unsafe_os", // includes: unsafe_{linux}
        feature = "unsafe_linux",
        feature = "unsafe_str",
        feature = "unsafe_string",
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

extern crate devela_macros;

/* root modules */

#[cfg(not(feature = "ascii"))]
pub(crate) mod ascii; // the "ascii" feature is disabled
#[cfg(feature = "ascii")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ascii")))]
pub mod ascii;

#[cfg(not(feature = "char"))]
pub(crate) mod char; // the "char" feature is disabled
#[cfg(feature = "char")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "char")))]
pub mod char;

#[cfg(not(feature = "cmp"))]
pub(crate) mod cmp; // the "cmp" feature is disabled
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "cmp")))]
pub mod cmp;

pub mod codegen;

#[cfg(not(feature = "convert"))]
pub(crate) mod convert; // the "convert" feature is disabled
#[cfg(feature = "convert")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "convert")))]
pub mod convert;

// #[cfg(not(feature = "fmt"))]
// pub(crate) mod fmt; // the "fmt" feature is disabled
#[cfg_attr(feature = "nightly", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
pub mod fmt;

#[cfg(not(feature = "mem"))]
pub(crate) mod mem; // the "mem" feature is disabled
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
#[cfg(feature = "mem")]
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
    pub use super::{
        codegen::all::*, num::*, ops::*, option::*, os::all::*, path::*, result::*, slice::*,
        str::*, string::all::*, sync::all::*,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::thread::*;

    #[doc(inline)]
    pub use devela_macros::{cif, compile, compile_attr};
}

/// The common prelude.
pub mod prelude {
    #[cfg(feature = "convert")]
    pub use crate::convert::{FromPrimitives, IntoPrimitives};

    pub use crate::{
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
