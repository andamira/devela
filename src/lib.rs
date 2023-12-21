// devela::lib
//
//! A highly integrated layer for Rust development.
//

// warnings
#![warn(missing_docs, clippy::all)]
#![cfg_attr(not(feature = "deps"), allow(rustdoc::broken_intra_doc_links))]
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
        feature = "unsafe_code", feature = "unsafe_color", feature = "unsafe_data",
        feature = "unsafe_io", feature = "unsafe_math", feature = "unsafe_mem",
        feature = "unsafe_result", feature = "unsafe_task", feature = "unsafe_text",
        feature = "unsafe_time",
    )
))]
compile_error!("You can't enable `safe` and `unsafe*` features at the same time.");

/* root modules */

#[cfg(any(feature = "color", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "color")))]
pub mod color;
#[cfg(all(not(feature = "color"), not(test)))]
pub(crate) mod color; // the "color" feature is disabled

#[cfg(any(feature = "data", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod data;
#[cfg(all(not(feature = "data"), not(test)))]
pub(crate) mod data; // the "data" feature is disabled

#[cfg(any(feature = "io", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "io")))]
pub mod io;
#[cfg(all(not(feature = "io"), not(test)))]
pub(crate) mod io; // the "io" feature is disabled

#[cfg(any(feature = "math", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "math")))]
pub mod math;
#[cfg(all(not(feature = "math"), not(test)))]
pub(crate) mod math; // the "math" feature is disabled

#[cfg(any(feature = "mem", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub mod mem;
#[cfg(all(not(feature = "mem"), not(test)))]
pub(crate) mod mem; // the "mem" feature is disabled

#[cfg(any(feature = "code", test))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
pub mod code;
#[cfg(all(not(feature = "code"), not(test)))]
pub(crate) mod code; // the "code" feature is disabled

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
/// Note that any item tagged with [`dep`] can also be enabled by
/// manually enabling the associated optional dependency.
pub mod all {
    #[doc(inline)]
    pub use super::{
        code::all::*, color::all::*, data::all::*, io::all::*, math::all::*,
        mem::all::*, result::all::*, task::all::*, text::all::*, time::all::*,
    };
}

/// The common prelude.
pub mod prelude {
    #[cfg(feature = "data")]
    pub use crate::data::{
        bit::BitOps,
        convert::{CastPrimitives, FromPrimitives, IntoPrimitives},
        AnyExt, DataCollection,
    };

    #[doc(no_inline)]
    #[cfg(all(feature = "io", feature = "std"))] // IMPROVE: no_std
    pub use crate::io::{BufRead, Read, Seek, Write};

    #[cfg(feature = "mem")]
    pub use crate::mem::{
        slice::{SliceExt, SliceExtMut},
        {BitSize, Mem, Size},
    };

    #[cfg(feature = "math")]
    pub use crate::math::num::{FloatOps, Num, NumRef};

    #[cfg(feature = "result")]
    pub use crate::result::{Also, Apply, OptionExt, ResultExt};

    #[cfg(feature = "text")]
    pub use crate::text::StrExt;
    #[cfg(all(feature = "text", feature = "alloc"))] // IMPROVE: no_alloc
    pub use crate::text::StringExt;
}

/// Optional external dependencies.
pub mod _dep;

/// Documentation on features.
pub mod __doc {
    #![cfg_attr(not(feature = "full"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./Doc.md")]
}
