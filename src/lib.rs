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
        feature = "unsafe_data", feature = "unsafe_convert",
        feature = "unsafe_mem", feature = "unsafe_num",
        //
        feature = "unsafe_os", // includes: unsafe_{linux, term}
            feature = "unsafe_linux", feature = "unsafe_term",
        //
        feature = "unsafe_path", feature = "unsafe_result",
        feature = "unsafe_task", feature = "unsafe_text",
        feature = "unsafe_time",
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

#[cfg(any(feature = "data", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod data;
#[cfg(all(not(feature = "data"), not(test)))]
pub(crate) mod data; // the "data" feature is disabled

#[cfg(any(feature = "convert", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "convert")))]
pub mod convert;
#[cfg(all(not(feature = "convert"), not(test)))]
pub(crate) mod convert; // the "convert" feature is disabled

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

#[cfg(any(feature = "task", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "task")))]
pub mod task;
#[cfg(all(not(feature = "task"), not(test)))]
pub(crate) mod task; // the "task" feature is disabled

#[cfg(any(feature = "text", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "text")))]
pub mod text;
#[cfg(all(not(feature = "text"), not(test)))]
pub(crate) mod text; // the "text" feature is disabled

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
    #[cfg(feature = "data")]
    pub use super::data::all::*;

    #[doc(inline)]
    #[cfg(feature = "convert")]
    pub use super::convert::all::*;

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::mem::all::*;

    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::num::all::*;

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
    #[cfg(feature = "task")]
    pub use super::task::all::*;

    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use super::text::all::*;

    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use super::time::all::*;
}

/// The common prelude.
pub mod prelude {
    #[cfg(feature = "any")]
    pub use crate::any::AnyExt;

    #[cfg(feature = "data")]
    pub use crate::data::{Collection, SliceExt, SliceExtMut};

    #[cfg(feature = "convert")]
    pub use crate::convert::{FromPrimitives, IntoPrimitives};

    #[cfg(feature = "mem")]
    pub use crate::mem::{BitSize, Mem, Size};

    #[cfg(feature = "num")]
    pub use crate::num::{Num, NumRef};

    #[cfg(feature = "result")]
    pub use crate::result::{Also, Apply, OptionExt, ResultExt};

    #[cfg(feature = "text")]
    pub use crate::text::StrExt;
    #[cfg(all(feature = "text", feature = "alloc"))] // IMPROVE
    pub use crate::text::StringExt;
}

/// Optional external dependencies.
pub mod depend;

/// Documentation on features.
pub mod __doc {
    #![doc = include_str!("./Doc.md")]
}
