// devela::lib
//
//! Development extensions for the Rust Standard Library.
//

// warnings
#![warn(missing_docs, clippy::all)]
#![allow(
    clippy::module_inception, // allow modules with the same name as its parent
    clippy::wrong_self_convention, // allow `is_` methods having an owned self
)]
// nightly, safety, environment
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]

/// <span class='stab portability' title='re-exported `alloc`'>`alloc`</span>
/// *Re-exported Rust `alloc` library environment.*
#[doc(inline)]
#[cfg(feature = "alloc")]
pub extern crate alloc as _alloc;
/// <span class='stab portability' title='re-exported `core`'>`core`</span>
/// *Re-exported Rust `core` library environment.*
#[doc(inline)]
pub use ::core as _core;
/// <span class='stab portability' title='re-exported `std`'>`std`</span>
/// *Re-exported Rust `std` library environment.*
#[cfg(feature = "std")]
#[doc(inline)]
pub use ::std as _std;

// safeguarding: environment, safety
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(
    feature = "safe",
    any(
        feature = "unsafe", // includes all below
        feature = "unsafe_any", feature = "unsafe_cmp", feature = "unsafe_codegen",
        feature = "unsafe_collections", feature = "unsafe_convert",
        feature = "unsafe_future", feature = "unsafe_mem", feature = "unsafe_num",
        feature = "unsafe_ops", feature = "unsafe_option",
        //
        feature = "unsafe_os", // includes: unsafe_{linux, term}
            feature = "unsafe_linux", feature = "unsafe_term",
        //
        feature = "unsafe_path", feature = "unsafe_result", feature = "unsafe_slice",
        feature = "unsafe_string", feature = "unsafe_sync",
        feature = "unsafe_task", feature = "unsafe_thread", feature = "unsafe_time",
    )
))]
compile_error!("You can't enable `safe` and `unsafe*` features at the same time.");

/* root modules */

#[cfg(any(feature = "any", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "any")))]
pub mod any;
#[cfg(all(not(feature = "any"), not(test)))]
pub(crate) mod any; // the "any" feature is disabled

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

#[cfg(any(feature = "collections", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "collections")))]
pub mod collections;
#[cfg(all(not(feature = "collections"), not(test)))]
pub(crate) mod collections; // the "collections" feature is disabled

#[cfg(any(feature = "convert", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "convert")))]
pub mod convert;
#[cfg(all(not(feature = "convert"), not(test)))]
pub(crate) mod convert; // the "convert" feature is disabled

#[cfg(any(feature = "future", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "future")))]
pub mod future;
#[cfg(all(not(feature = "future"), not(test)))]
pub(crate) mod future; // the "future" feature is disabled

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

pub mod os;

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

#[cfg(any(feature = "task", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub mod task;
#[cfg(all(not(feature = "task"), not(test)))]
pub(crate) mod task; // the "task" feature is disabled

#[cfg(any(feature = "thread", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "thread")))]
pub mod thread;
// #[cfg(all(not(feature = "thread"), not(test)))]
// pub(crate) mod thread; // the "thread" feature is disabled

#[cfg(any(feature = "time", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "time")))]
pub mod time;
#[cfg(all(not(feature = "time"), not(test)))]
pub(crate) mod time; // the "time" feature is disabled

/// All items are flat re-exported here.
///
/// Note that any item tagged with [`depend`] can also be enabled by
/// manually enabling the associated optional dependency.
pub mod all {
    #[doc(inline)]
    #[cfg(feature = "any")]
    pub use super::any::all::*;

    #[doc(inline)]
    #[cfg(feature = "cmp")]
    pub use super::cmp::all::*;

    #[doc(inline)]
    #[cfg(feature = "codegen")]
    pub use super::codegen::all::*;

    #[doc(inline)]
    #[cfg(feature = "collections")]
    pub use super::collections::all::*;

    #[doc(inline)]
    #[cfg(feature = "convert")]
    pub use super::convert::all::*;

    #[doc(inline)]
    #[cfg(feature = "future")]
    pub use super::future::all::*;

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::mem::all::*;

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
    // no `os` feature, just for each platform submodule
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
    #[cfg(feature = "string")]
    pub use super::string::all::*;

    #[doc(inline)]
    #[cfg(feature = "sync")]
    pub use super::sync::all::*;

    #[doc(inline)]
    #[cfg(feature = "task")]
    pub use super::task::all::*;

    #[allow(unused)] // only contains "std" for now
    #[doc(inline)]
    #[cfg(feature = "thread")]
    pub use super::thread::*;

    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use super::time::all::*;
}

/// The common prelude.
pub mod prelude {
    #[doc(inline)]
    #[cfg(feature = "any")]
    pub use crate::any::AnyExt;

    #[doc(inline)]
    #[cfg(feature = "convert")]
    pub use crate::convert::all::{FromPrimitives, IntoPrimitives};

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use crate::mem::all::{BitSize, Mem, Size};

    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use crate::num::all::{Num, NumRef};

    #[doc(inline)]
    #[cfg(feature = "ops")]
    pub use crate::ops::all::{Also, Apply};

    #[doc(inline)]
    #[cfg(feature = "slice")]
    pub use crate::slice::all::{SliceExt, SliceExtMut};

    #[doc(inline)]
    #[cfg(feature = "option")]
    pub use crate::option::OptionExt;

    #[doc(inline)]
    #[cfg(feature = "result")]
    pub use crate::result::ResultExt;

    #[doc(inline)]
    #[cfg(all(feature = "string", feature = "alloc"))]
    pub use crate::string::{StrExt, StringExt};
}

/// Optional external dependencies.
pub mod depend;

/// Documentation on features.
pub mod __doc {
    #![doc = include_str!("./Doc.md")]
}
