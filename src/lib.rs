// devela::lib
//
//! An all-encompassing, highly integrated layer for Rust development.
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

// safeguarding: environment, safety
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(
    feature = "safe",
    any(feature = "unsafe", // includes all below:
        feature = "unsafe_data", feature = "unsafe_mem", feature = "unsafe_num",
        feature = "unsafe_text", feature = "unsafe_work",
        feature = "unsafe_os",
        feature = "unsafe_ui", // includes all below:
            feature = "unsafe_ui_term",
    )
))]
compile_error!("You can't enable `safe` and `unsafe*` features at the same time.");

/* root modules */

#[cfg(any(feature = "code", test))]
pub mod code;
#[cfg(not(any(feature = "code", test)))]
pub(crate) mod code; // the "code" feature is disabled

#[cfg(any(feature = "data", test))]
pub mod data;
#[cfg(not(any(feature = "data", test)))]
pub(crate) mod data; // the "data" feature is disabled

#[cfg(any(feature = "error", test))]
pub mod error;
#[cfg(not(any(feature = "error", test)))]
pub(crate) mod error; // the "error" feature is disabled

#[cfg(any(feature = "io", test))]
pub mod io;
#[cfg(not(any(feature = "io", test)))]
pub(crate) mod io; // the "io" feature is disabled

#[cfg(any(feature = "mem", test))]
pub mod mem;
#[cfg(not(any(feature = "mem", test)))]
pub(crate) mod mem; // the "mem" feature is disabled

#[cfg(any(feature = "num", test))]
pub mod num;
#[cfg(not(any(feature = "num", test)))]
pub(crate) mod num; // the "num" feature is disabled

#[cfg(any(feature = "os", feature = "os_term", test))]
pub mod os;
#[cfg(not(any(feature = "os", feature = "os_term", test)))]
pub(crate) mod os; // the "os" features are disabled

#[cfg(any(feature = "render", test))]
pub mod render;
#[cfg(not(any(feature = "render", test)))]
pub(crate) mod render; // the "render" feature is disabled

#[cfg(any(feature = "text", test))]
pub mod text;
#[cfg(not(any(feature = "text", test)))]
pub(crate) mod text; // the "text" feature is disabled

#[cfg(any(feature = "time", test))]
pub mod time;
#[cfg(not(any(feature = "time", test)))]
pub(crate) mod time; // the "time" feature is disabled

#[cfg(any(feature = "ui", test))]
pub mod ui;
#[cfg(not(any(feature = "ui", test)))]
pub(crate) mod ui; // the "ui" feature is disabled

#[cfg(any(feature = "work", test))]
pub mod work;
#[cfg(not(any(feature = "work", test)))]
pub(crate) mod work; // the "work" feature is disabled

/// All items are flat re-exported here.
///
/// Note that any item tagged with [`dep`] can also be enabled by
/// manually enabling the associated optional dependency.
pub mod all {
    #[allow(unused_imports)]
    #[doc(inline)]
    pub use super::{
        code::all::*, data::all::*, error::all::*, io::all::*, mem::all::*, num::all::*,
        os::all::*, render::all::*, text::all::*, time::all::*, ui::all::*, work::all::*,
    };
}

/// Dependencies.
pub mod _deps;

/// Documentation.
pub mod _docs {
    #![cfg_attr(not(feature = "full"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./_docs/features.md")]
}
