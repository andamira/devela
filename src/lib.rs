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
        feature = "unsafe_cmp",
        feature = "unsafe_convert",
        feature = "unsafe_num",
        feature = "unsafe_fmt"
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

extern crate devela_macros;

pub mod ascii;
pub mod char;
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
    pub use super::{
        ascii::*,
        char::*,
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

/// General documentation.
pub mod _doc {
    #![doc = include_str!("./Doc.md")]
}
