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
        feature = "unsafe_data", feature = "unsafe_error", feature = "unsafe_mem",
        feature = "unsafe_num", feature = "unsafe_text", feature = "unsafe_work",
        feature = "unsafe_os",
        feature = "unsafe_ui", // includes all below:
            feature = "unsafe_ui_term",
    )
))]
compile_error!("You can't enable `safe` and `unsafe*` features at the same time.");

/// Dependencies.
pub mod _deps;

/// Documentation.
pub mod _docs {
    #![cfg_attr(not(feature = "full"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./_docs/features.md")]
}

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

/* root modules */

pub mod code;
pub mod data;
pub mod error;
pub mod io;
pub mod mem;
pub mod num;
pub mod os;
pub mod render;
pub mod text;
pub mod time;
pub mod ui;
pub mod work;
