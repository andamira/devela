// devela::lib
//
//! Development extensions for the Rust Standard Library.
//

// warnings
#![warn(missing_docs, clippy::all)]
#![allow(clippy::wrong_self_convention)] // allow `is_` methods with owned self
// nightly, safety, environment
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
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
        feature = "unsafe_codegen",
        feature = "unsafe_convert",
        feature = "unsafe_fmt",
        feature = "unsafe_mem",
        feature = "unsafe_num",
        feature = "unsafe_ops",
        feature = "unsafe_option",
        feature = "unsafe_os", // includes: unsafe_{linux}
        feature = "unsafe_linux", //
        feature = "unsafe_path",
        feature = "unsafe_result",
        feature = "unsafe_slice",
        feature = "unsafe_str",
        feature = "unsafe_string",
        feature = "unsafe_sync",
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

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

#[cfg(any(feature = "codegen", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "codegen")))]
pub mod codegen;
#[cfg(all(not(feature = "codegen"), not(test)))]
pub(crate) mod codegen; // the "codegen" feature is disabled

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

#[cfg(any(feature = "option", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "option")))]
pub mod option;
#[cfg(all(not(feature = "option"), not(test)))]
pub(crate) mod option; // the "option" feature is disabled

#[cfg(any(feature = "os", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "os")))]
pub mod os;
#[cfg(all(not(feature = "os"), not(test)))]
pub(crate) mod os; // the "os" feature is disabled

#[cfg(any(feature = "path", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "path")))]
pub mod path;
#[cfg(all(not(feature = "path"), not(test)))]
pub(crate) mod path; // the "path" feature is disabled

#[cfg(any(feature = "result", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "result")))]
pub mod result;
#[cfg(all(not(feature = "result"), not(test)))]
pub(crate) mod result; // the "result" feature is disabled

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

#[cfg(any(feature = "thread", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "thread")))]
pub mod thread;
// #[cfg(all(not(feature = "thread"), not(test)))]
// pub(crate) mod thread; // the "thread" feature is disabled

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
    #[cfg(feature = "codegen")]
    pub use super::codegen::all::*;
    #[cfg(feature = "devela_macros")]
    #[doc(inline)]
    pub use devela_macros::{cif, compile, compile_attr};

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
    #[cfg(feature = "option")]
    pub use super::option::all::*;

    #[doc(inline)]
    #[cfg(feature = "os")]
    pub use super::os::all::*;

    #[doc(inline)]
    #[cfg(feature = "path")]
    pub use super::path::all::*;

    #[doc(inline)]
    #[cfg(feature = "result")]
    pub use super::result::all::*;

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
    #[cfg(feature = "thread")]
    pub use super::thread::*;
}

/// The common prelude.
pub mod prelude {
    #[doc(inline)]
    #[cfg(feature = "convert")]
    pub use crate::convert::all::{FromPrimitives, IntoPrimitives};

    #[doc(inline)]
    #[cfg(feature = "ops")]
    pub use crate::ops::all::{Also, Apply};

    #[doc(inline)]
    #[cfg(feature = "slice")]
    pub use crate::slice::all::{SliceExt, SliceExtMut};

    #[doc(inline)]
    #[cfg(feature = "str")]
    pub use crate::str::all::StrExt;

    #[doc(inline)]
    #[cfg(feature = "option")]
    pub use crate::option::OptionExt;

    #[doc(inline)]
    #[cfg(feature = "result")]
    pub use crate::result::ResultExt;

    #[doc(inline)]
    #[cfg(all(feature = "string", feature = "alloc"))]
    pub use crate::string::StringExt;
}

/// Documentation on features.
pub mod _doc {
    #![doc = include_str!("./Doc.md")]
}
